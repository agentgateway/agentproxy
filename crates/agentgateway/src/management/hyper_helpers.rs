// Originally derived from https://github.com/istio/ztunnel (Apache 2.0 licensed)

use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use agent_core::drain::DrainWatcher;
use bytes::Bytes;
use futures_util::TryFutureExt;
use http_body_util::Full;
use hyper::rt::Sleep;
use hyper::server::conn::{http1, http2};
use hyper::{Request, client};
use hyper_util::rt::{TokioExecutor, TokioTimer};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::Stream;
use tracing::{Instrument, debug, info, warn};

use crate::http::{Body, Response};

struct TokioTimeout<T> {
	inner: Pin<Box<tokio::time::Timeout<T>>>,
}

impl<T> Future for TokioTimeout<T>
where
	T: Future,
{
	type Output = Result<T::Output, tokio::time::error::Elapsed>;

	fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		self.inner.as_mut().poll(context)
	}
}

// Use TokioSleep to get tokio::time::Sleep to implement Unpin.
// see https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html
pub struct TokioSleep {
	pub inner: Pin<Box<tokio::time::Sleep>>,
}

impl Future for TokioSleep {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.inner.as_mut().poll(cx)
	}
}

// Use HasSleep to get tokio::time::Sleep to implement Unpin.
// see https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html

impl Sleep for TokioSleep {}

pub fn http2_server() -> http2::Builder<TokioExecutor> {
	let mut b = http2::Builder::new(TokioExecutor::new());
	b.timer(TokioTimer::new());
	b
}

pub fn http1_server() -> http1::Builder {
	let mut b = http1::Builder::new();
	b.timer(TokioTimer::new());
	b
}

pub fn http2_client() -> client::conn::http2::Builder<TokioExecutor> {
	let mut b = client::conn::http2::Builder::new(TokioExecutor::new());
	b.timer(TokioTimer::new());
	b
}

pub fn empty_response(code: hyper::StatusCode) -> Response {
	::http::Response::builder()
		.status(code)
		.body(Body::empty())
		.unwrap()
}

pub fn plaintext_response(code: hyper::StatusCode, body: String) -> Response {
	::http::Response::builder()
		.status(code)
		.header(hyper::header::CONTENT_TYPE, "text/plain")
		.body(body.into())
		.unwrap()
}

/// Server implements a generic HTTP server with the follow behavior:
/// * HTTP/1.1 plaintext only
/// * Draining
pub struct Server<S> {
	name: String,
	binds: Vec<TcpListener>,
	drain_rx: DrainWatcher,
	state: S,
}

impl<S> Server<S> {
	pub async fn bind(
		name: &str,
		addrs: crate::Address,
		drain_rx: DrainWatcher,
		s: S,
	) -> anyhow::Result<Self> {
		let mut binds = vec![];
		for addr in addrs.into_iter() {
			binds.push(TcpListener::bind(&addr).await?)
		}
		Ok(Server {
			name: name.to_string(),
			binds,
			drain_rx,
			state: s,
		})
	}

	pub fn address(&self) -> SocketAddr {
		self
			.binds
			.first()
			.expect("must have at least one address")
			.local_addr()
			.expect("local address must be ready")
	}

	pub fn state_mut(&mut self) -> &mut S {
		&mut self.state
	}

	pub fn spawn<F, R>(self, f: F)
	where
		S: Send + Sync + 'static,
		F: Fn(Arc<S>, Request<hyper::body::Incoming>) -> R + Send + Sync + 'static,
		R: Future<Output = Result<crate::http::Response, anyhow::Error>> + Send + 'static,
	{
		use futures_util::StreamExt as OtherStreamExt;
		let address = self.address();
		let drain = self.drain_rx;
		let state = Arc::new(self.state);
		let f = Arc::new(f);
		info!(
				%address,
				component=self.name,
				"listener established",
		);
		for bind in self.binds {
			let drain_stream = drain.clone();
			let drain_connections = drain.clone();
			let state = state.clone();
			let name = self.name.clone();
			let f = f.clone();
			tokio::spawn(async move {
				let stream = tokio_stream::wrappers::TcpListenerStream::new(bind);
				let mut stream = stream.take_until(Box::pin(drain_stream.wait_for_drain()));
				while let Some(Ok(socket)) = stream.next().await {
					socket.set_nodelay(true).unwrap();
					let drain = drain_connections.clone();
					let f = f.clone();
					let state = state.clone();
					tokio::spawn(async move {
						let serve = http1_server()
							.half_close(true)
							.header_read_timeout(Duration::from_secs(2))
							.max_buf_size(8 * 1024)
							.serve_connection(
								hyper_util::rt::TokioIo::new(socket),
								hyper::service::service_fn(move |req| {
									let state = state.clone();

									// Failures would abort the whole connection; we just want to return an HTTP error
									f(state, req).or_else(|err| async move {
										Ok::<_, Infallible>(
											::http::Response::builder()
												.status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
												.body(crate::http::Body::new(err.to_string()))
												.expect("builder with known status code should not fail"),
										)
									})
								}),
							);
						// Wait for drain to signal or connection serving to complete
						match futures_util::future::select(Box::pin(drain.wait_for_drain()), serve).await {
							// We got a shutdown request. Start gracful shutdown and wait for the pending requests to complete.
							futures_util::future::Either::Left((_shutdown, mut serve)) => {
								let drain = std::pin::Pin::new(&mut serve);
								drain.graceful_shutdown();
								serve.await
							},
							// Serving finished, just return the result.
							futures_util::future::Either::Right((serve, _shutdown)) => serve,
						}
					});
				}
				info!(
						%address,
						component=name,
						"listener drained",
				);
			});
		}
	}
}
