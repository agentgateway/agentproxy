// Portions of this code are heavily inspired from https://github.com/Kuadrant/wasm-shim/
// Under Apache 2.0 license (https://github.com/Kuadrant/wasm-shim/blob/main/LICENSE)

use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use axum_core::body::Body;
use bytes::Bytes;
use cel_interpreter::extractors::{Arguments, This};
use cel_interpreter::objects::{Key, Map, TryIntoValue, ValueType};
use cel_interpreter::{Context, ExecutionError, FunctionContext, Program, ResolveResult, Value};
use cel_parser::{Expression as CelExpression, ParseError};
use http::Request;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize, Serializer};

use crate::http::backendtls::{BackendTLS, LocalBackendTLS};
use crate::http::jwt::Claims;
use crate::json;
use crate::serdes::*;
use crate::telemetry::log::CelLogging;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("execution: {0}")]
	Resolve(#[from] ExecutionError),
	#[error("parse: {0}")]
	Parse(#[from] ParseError),
	#[error("variable: {0}")]
	Variable(String),
}

impl From<Box<dyn std::error::Error>> for Error {
	fn from(value: Box<dyn std::error::Error>) -> Self {
		Self::Variable(value.to_string())
	}
}

pub const REQUEST_ATTRIBUTE: &str = "request";
pub const REQUEST_BODY_ATTRIBUTE: &str = "request.body";
pub const RESPONSE_ATTRIBUTE: &str = "response";
pub const JWT_ATTRIBUTE: &str = "jwt";
pub const MCP_ATTRIBUTE: &str = "mcp";

pub struct Expression {
	attributes: HashSet<String>,
	expression: CelExpression,
	original_expression: String,
}

impl Serialize for Expression {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.original_expression)
	}
}

impl Debug for Expression {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Expression")
			.field("expression", &self.original_expression)
			.finish()
	}
}

fn root_context() -> Arc<Context<'static>> {
	let mut ctx = Context::default();
	ctx.add_function("json", fns::json_parse);
	Arc::new(ctx)
}

static ROOT_CONTEXT: Lazy<Arc<Context<'static>>> = Lazy::new(|| Arc::new(Context::default()));

#[derive(Debug)]
pub struct ContextBuilder {
	pub attributes: HashSet<String>,
	pub context: ExpressionContext,
}

impl Default for ContextBuilder {
	fn default() -> Self {
		Self::new()
	}
}

impl ContextBuilder {
	pub fn new() -> Self {
		Self {
			attributes: Default::default(),
			context: Default::default(),
		}
	}
	/// register_expression registers the given expressions attributes as required attributes.
	/// Callers MUST call this for each expression they wish to call with the context if they want correct results.
	pub fn register_expression(&mut self, expression: &Expression) {
		self
			.attributes
			.extend(expression.attributes.iter().cloned());
	}
	pub fn with_request_body(&mut self, body: Bytes) {
		let Some(r) = &mut self.context.request else {
			return;
		};
		r.body = Some(body);
	}
	pub fn with_request(&mut self, req: &crate::http::Request) -> bool {
		if !self.attributes.contains(REQUEST_ATTRIBUTE) {
			return false;
		}
		self.context.request = Some(RequestContext {
			method: req.method().clone(),
			// TODO: split headers and the rest?
			headers: req.headers().clone(),
			uri: req.uri().clone(),
			body: None,
		});
		self.attributes.contains(REQUEST_BODY_ATTRIBUTE)
	}
	pub fn with_response(&mut self, resp: &crate::http::Response) {
		if !self.attributes.contains(RESPONSE_ATTRIBUTE) {
			return;
		}
		self.context.response = Some(ResponseContext {
			code: resp.status(),
		})
	}

	pub fn with_jwt(&mut self, info: &Claims) {
		if !self.attributes.contains(JWT_ATTRIBUTE) {
			return;
		}
		self.context.jwt = Some(info.clone())
	}

	pub fn build_with_mcp(
		&self,
		mcp: Option<&crate::mcp::rbac::ResourceType>,
	) -> Result<Executor<'static>, Error> {
		let mut ctx: Context<'static> = ROOT_CONTEXT.new_inner_scope();

		let ExpressionContext {
			request,
			response,
			jwt,
		} = &self.context;

		ctx.add_variable_from_value(REQUEST_ATTRIBUTE, opt_to_value(request)?);
		ctx.add_variable_from_value(RESPONSE_ATTRIBUTE, opt_to_value(response)?);
		ctx.add_variable_from_value(JWT_ATTRIBUTE, opt_to_value(jwt)?);
		ctx.add_variable_from_value(MCP_ATTRIBUTE, opt_to_value(&mcp)?);

		Ok(Executor { ctx })
	}

	pub fn build(&self) -> Result<Executor<'static>, Error> {
		self.build_with_mcp(None)
	}
}

impl Executor<'_> {
	pub fn eval(&self, expr: &Expression) -> Result<Value, Error> {
		Ok(Value::resolve(&expr.expression, &self.ctx)?)
	}
	pub fn eval_bool(&self, expr: &Expression) -> bool {
		match self.eval(expr) {
			Ok(Value::Bool(b)) => b,
			_ => false,
		}
	}
}

pub struct Executor<'a> {
	ctx: Context<'a>,
}
impl Expression {
	pub fn new(original_expression: impl Into<String>) -> Result<Self, Error> {
		let original_expression = original_expression.into();
		let expression = cel_parser::parse(&original_expression)?;

		let mut props = Vec::with_capacity(5);
		properties(&expression, &mut props, &mut Vec::default());

		// For now we only look at the first level. We could be more precise
		let mut attributes: HashSet<String> = props
			.into_iter()
			.filter_map(|tokens| match tokens.as_slice() {
				["request", "body", ..] => Some(REQUEST_BODY_ATTRIBUTE.to_string()),
				[first, ..] => Some(first.to_string()),
				_ => None,
			})
			.collect();

		Ok(Self {
			attributes,
			expression,
			original_expression,
		})
	}
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ExpressionContext {
	pub request: Option<RequestContext>,
	pub response: Option<ResponseContext>,
	pub jwt: Option<Claims>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RequestContext {
	#[serde(with = "http_serde::method")]
	pub method: ::http::Method,

	#[serde(with = "http_serde::uri")]
	pub uri: ::http::Uri,

	#[serde(with = "http_serde::header_map")]
	pub headers: ::http::HeaderMap,

	pub body: Option<Bytes>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseContext {
	#[serde(with = "http_serde::status_code")]
	pub code: ::http::StatusCode,
}

fn create_context<'a>() -> Context<'a> {
	Context::default()
}

fn properties<'e>(exp: &'e CelExpression, all: &mut Vec<Vec<&'e str>>, path: &mut Vec<&'e str>) {
	match exp {
		CelExpression::Arithmetic(e1, _, e2)
		| CelExpression::Relation(e1, _, e2)
		| CelExpression::Ternary(e1, _, e2)
		| CelExpression::Or(e1, e2)
		| CelExpression::And(e1, e2) => {
			properties(e1, all, path);
			properties(e2, all, path);
		},
		CelExpression::Unary(_, e) => {
			properties(e, all, path);
		},
		CelExpression::Member(e, a) => {
			if let cel_parser::Member::Attribute(attr) = &**a {
				path.insert(0, attr.as_str())
			}
			properties(e, all, path);
		},
		CelExpression::FunctionCall(_, target, args) => {
			// The attributes of the values returned by functions are skipped.
			path.clear();
			if let Some(target) = target {
				properties(target, all, path);
			}
			for e in args {
				properties(e, all, path);
			}
		},
		CelExpression::List(e) => {
			for e in e {
				properties(e, all, path);
			}
		},
		CelExpression::Map(v) => {
			for (e1, e2) in v {
				properties(e1, all, path);
				properties(e2, all, path);
			}
		},
		CelExpression::Atom(_) => {},
		CelExpression::Ident(v) => {
			if !path.is_empty() {
				path.insert(0, v.as_str());
				all.push(path.clone());
				path.clear();
			}
		},
	}
}

pub struct Attribute {
	path: Path,
	cel_type: Option<ValueType>,
}

impl Debug for Attribute {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Attribute {{ {:?} }}", self.path)
	}
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Path {
	tokens: Vec<String>,
}

impl Display for Path {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.tokens
				.iter()
				.map(|t| t.replace('.', "\\."))
				.collect::<Vec<String>>()
				.join(".")
		)
	}
}

impl Debug for Path {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "path: {:?}", self.tokens)
	}
}

impl From<&str> for Path {
	fn from(value: &str) -> Self {
		let mut token = String::new();
		let mut tokens: Vec<String> = Vec::new();
		let mut chars = value.chars();
		while let Some(ch) = chars.next() {
			match ch {
				'.' => {
					tokens.push(token);
					token = String::new();
				},
				'\\' => {
					if let Some(next) = chars.next() {
						token.push(next);
					}
				},
				_ => token.push(ch),
			}
		}
		tokens.push(token);

		Self { tokens }
	}
}

impl Path {
	pub fn new<T: Into<String>>(tokens: Vec<T>) -> Self {
		Self {
			tokens: tokens.into_iter().map(|i| i.into()).collect(),
		}
	}
	pub fn tokens(&self) -> Vec<&str> {
		self.tokens.iter().map(String::as_str).collect()
	}
}

fn opt_to_value<S: Serialize>(v: &Option<S>) -> Result<Value, Error> {
	Ok(v.as_ref().map(to_value).transpose()?.unwrap_or(Value::Null))
}

fn to_value(v: impl Serialize) -> Result<Value, Error> {
	cel_interpreter::to_value(v).map_err(|e| Error::Variable(e.to_string()))
}

mod fns {
	use std::sync::Arc;

	use cel_interpreter::{FunctionContext, ResolveResult, Value};

	use crate::cel::to_value;

	pub fn json_parse(ftx: &FunctionContext, v: Value) -> ResolveResult {
		let sv = match v {
			Value::String(b) => serde_json::from_str(b.as_str()),
			Value::Bytes(b) => serde_json::from_slice(b.as_ref()),
			_ => return Err(ftx.error("invalid type")),
		};
		let sv: serde_json::Value = sv.map_err(|e| ftx.error(e))?;
		to_value(sv).map_err(|e| ftx.error(e))
	}
}

#[cfg(any(test, feature = "internal_benches"))]
pub mod tests {
	use std::collections::HashMap;
	use std::fs::File;
	use std::io::Write;
	use std::net::{IpAddr, Ipv4Addr, SocketAddr};
	use std::time::Duration;

	use agent_core::strng;
	use divan::Bencher;
	use http::Method;

	use super::*;
	use crate::http::Body;
	use crate::store::Stores;
	use crate::types::agent::{Listener, ListenerProtocol, PathMatch, Route, RouteMatch, RouteSet};
	fn simple(expr: &str, req: crate::http::Request) -> Result<Value, Error> {
		let mut cb = ContextBuilder::new();
		let exp = Expression::new(expr)?;
		cb.register_expression(&exp);
		cb.with_request(&req);
		let exec = cb.build()?;
		exec.eval(&exp)
	}

	#[test]
	fn test_eval() {
		let expr = Arc::new(Expression::new(r#"request.method"#).unwrap());
		let ctx = root_context();
		let req = ::http::Request::builder()
			.method(Method::GET)
			.header("x-example", "value")
			.body(Body::empty())
			.unwrap();
		let mut cb = ContextBuilder::new();
		cb.register_expression(&expr);
		cb.with_request(&req);
		let exec = cb.build().unwrap();

		exec.eval(&expr);
	}

	#[test]
	fn expression() {
		let expr = r#"request.method == "GET" && request.headers["x-example"] == "value""#;
		let req = ::http::Request::builder()
			.method(Method::GET)
			.uri("http://example.com")
			.header("x-example", "value")
			.body(Body::empty())
			.unwrap();
		assert_eq!(Value::Bool(true), simple(expr, req).unwrap());
	}

	#[divan::bench]
	fn bench_native(b: Bencher) {
		let req = ::http::Request::builder()
			.method(Method::GET)
			.header("x-example", "value")
			.body(http_body_util::Empty::<Bytes>::new())
			.unwrap();
		b.bench(|| {
			divan::black_box(req.method());
		});
	}

	#[divan::bench]
	fn bench_native_map(b: Bencher) {
		let req = ::http::Request::builder()
			.method(Method::GET)
			.header("x-example", "value")
			.body(http_body_util::Empty::<Bytes>::new())
			.unwrap();
		let map = HashMap::from([(
			"request".to_string(),
			HashMap::from([("method".to_string(), "GET".to_string())]),
		)]);

		with_profiling("native", || {
			b.bench(|| {
				divan::black_box(map.get("request").unwrap().get("method").unwrap());
			});
		})
	}

	#[macro_export]
	macro_rules! function {
		() => {{
			fn f() {}
			fn type_name_of<T>(_: T) -> &'static str {
				std::any::type_name::<T>()
			}
			let name = type_name_of(f);
			let name = &name[..name.len() - 3].to_string();
			name.strip_suffix("::with_profiling").unwrap().to_string()
		}};
	}

	fn with_profiling(name: &str, f: impl FnOnce()) {
		use pprof::protos::Message;
		let guard = pprof::ProfilerGuardBuilder::default()
			.frequency(1000)
			// .blocklist(&["libc", "libgcc", "pthread", "vdso"])
			.build()
			.unwrap();

		f();

		let report = guard.report().build().unwrap();
		let profile = report.pprof().unwrap();

		let mut body = profile.write_to_bytes().unwrap();
		File::create(format!("/tmp/pprof-{}::{name}", function!()))
			.unwrap()
			.write_all(&body)
			.unwrap()
	}

	#[divan::bench]
	fn bench_lookup(b: Bencher) {
		let expr = Arc::new(Expression::new(r#"request.method"#).unwrap());
		let ctx = root_context();
		let req = ::http::Request::builder()
			.method(Method::GET)
			.header("x-example", "value")
			.body(Body::empty())
			.unwrap();
		let mut cb = ContextBuilder::new();
		cb.register_expression(&expr);
		cb.with_request(&req);
		let exec = cb.build().unwrap();

		with_profiling("lookup", || {
			b.bench(|| {
				exec.eval(&expr);
			});
		})
	}

	#[divan::bench]
	fn bench_with_response(b: Bencher) {
		let expr = Arc::new(
			Expression::new(r#"response.status == 200 && response.headers["x-example"] == "value""#)
				.unwrap(),
		);
		b.with_inputs(|| {
			::http::Response::builder()
				.status(200)
				.header("x-example", "value")
				.body(Body::empty())
				.unwrap()
		})
		.bench_refs(|r| {
			let mut cb = ContextBuilder::new();
			cb.register_expression(&expr);
			cb.with_response(r);
			let exec = cb.build()?;
			exec.eval(&expr)
		});
	}

	#[divan::bench]
	fn bench(b: Bencher) {
		let expr = Arc::new(Expression::new(r#"1 + 2 == 3"#).unwrap());
		b.with_inputs(|| {
			::http::Response::builder()
				.status(200)
				.header("x-example", "value")
				.body(Body::empty())
				.unwrap()
		})
		.bench_refs(|r| {
			let mut cb = ContextBuilder::new();
			cb.register_expression(&expr);
			cb.with_response(r);
			let exec = cb.build()?;
			exec.eval(&expr)
		});
	}
}
