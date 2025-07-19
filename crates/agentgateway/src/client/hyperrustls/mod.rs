use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::{fmt, io};

use http::Uri;
use hyper::rt;
use hyper_util::rt::TokioIo;
use itertools::Itertools;
use rustls_pki_types::ServerName;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use tower::Service;
use tracing::debug;

use crate::transport::stream;
use crate::transport::stream::Socket;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// A Connector for the `https` scheme.
#[derive(Clone)]
pub struct HttpsConnector {
	pub tls_config: Arc<rustls::ClientConfig>,
	pub server_name: ServerName<'static>,
}

impl Service<SocketAddr> for HttpsConnector {
	type Response = Socket;
	type Error = BoxError;

	#[allow(clippy::type_complexity)]
	type Future = Pin<Box<dyn Future<Output = Result<Socket, BoxError>> + Send>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, dst: SocketAddr) -> Self::Future {
		let cfg = self.tls_config.clone();
		let hostname = self.server_name.clone();

		debug!(%dst, ?hostname,
			alpn=?cfg.alpn_protocols.iter().map(|bytes| String::from_utf8_lossy(bytes.as_slice())).collect_vec(),
			"connecting tls");

		let connecting_future = Socket::dial(dst);
		Box::pin(async move {
			let tcp = connecting_future.await?;
			let (ext, counter, tcp) = tcp.into_parts();
			let tls = TlsConnector::from(cfg)
				.connect(hostname, Box::new(tcp))
				.await
				.map_err(io::Error::other)?;
			let socket = Socket::from_tls(stream::Extension::new(), counter, tls.into())?;
			Ok(socket)
		})
	}
}

impl fmt::Debug for HttpsConnector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("HttpsConnector").finish()
	}
}
