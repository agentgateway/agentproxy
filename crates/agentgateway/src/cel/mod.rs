// Portions of this code are heavily inspired from https://github.com/Kuadrant/wasm-shim/
// Under Apache 2.0 license (https://github.com/Kuadrant/wasm-shim/blob/main/LICENSE)

use crate::http::jwt::Claims;
use crate::serdes::*;
use axum_core::body::Body;
use bytes::Bytes;
use cel_interpreter::extractors::{Arguments, This};
use cel_interpreter::objects::{Key, Map, TryIntoValue, ValueType};
use cel_interpreter::{Context, ExecutionError, Program, ResolveResult, Value};
use cel_parser::{Expression as CelExpression, ParseError};
use http::Request;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

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

const REQUEST_ATTRIBUTE: &str = "request";
const RESPONSE_ATTRIBUTE: &str = "response";
const JWT_ATTRIBUTE: &str = "jwt";
const MCP_ATTRIBUTE: &str = "mcp";

pub struct Expression {
	attributes: HashSet<String>,
	expression: CelExpression,
	original_expression: String,
	root_context: Context<'static>,
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

pub struct ExpressionCall {
	expression: Arc<Expression>,
	context: ExpressionContext,
}

impl Debug for ExpressionCall {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExpressionCall").finish()
	}
}
impl ExpressionCall {}

impl ExpressionCall {
	pub fn new(expression: &str) -> Result<Self, Error> {
		let exp = Expression::new(expression)?;
		Ok(ExpressionCall {
			expression: Arc::new(exp),
			context: ExpressionContext::default(),
		})
	}
	pub fn from_expression(expression: Arc<Expression>) -> Self {
		ExpressionCall {
			expression,
			context: ExpressionContext::default(),
		}
	}
	pub fn with_request(&mut self, req: &crate::http::Request) {
		if !self.expression.attributes.contains(REQUEST_ATTRIBUTE) {
			return;
		}
		self.context.request = Some(RequestContext {
			method: req.method().clone(),
			// TODO: split headers and the rest?
			headers: req.headers().clone(),
			uri: req.uri().clone(),
		})
	}
	pub fn with_response(&mut self, resp: &crate::http::Response) {
		if !self.expression.attributes.contains(RESPONSE_ATTRIBUTE) {
			return;
		}
		self.context.response = Some(ResponseContext {
			code: resp.status(),
		})
	}

	pub fn with_jwt(&mut self, info: &Claims) {
		if !self.expression.attributes.contains(JWT_ATTRIBUTE) {
			return;
		}
		self.context.jwt = Some(info.clone())
	}

	pub fn with_mcp(&mut self, info: &crate::mcp::rbac::ResourceType) {
		if !self.expression.attributes.contains(MCP_ATTRIBUTE) {
			return;
		}
		self.context.mcp = Some(info.clone())
	}

	pub fn eval(&self) -> Result<Value, Error> {
		self.expression.eval(&self.context)
	}
	pub fn eval_bool(&self) -> bool {
		match self.expression.eval(&self.context) {
			Ok(Value::Bool(b)) => b,
			_ => false,
		}
	}
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
			.filter_map(|tokens| tokens.first().map(|s| s.to_string()))
			.collect();

		Ok(Self {
			attributes,
			expression,
			original_expression,
			root_context: Context::default(),
		})
	}

	fn eval(&self, ec: &ExpressionContext) -> Result<Value, Error> {
		let mut ctx = self.root_context.new_inner_scope();

		let ExpressionContext {
			request,
			response,
			jwt,
			mcp,
		} = ec;

		ctx.add_variable_from_value("request", opt_to_value(request)?);
		ctx.add_variable_from_value("response", opt_to_value(response)?);
		ctx.add_variable_from_value("jwt", opt_to_value(jwt)?);
		ctx.add_variable_from_value("mcp", opt_to_value(mcp)?);

		Ok(Value::resolve(&self.expression, &ctx)?)
	}
}

#[derive(Clone, Debug, Default, Serialize)]
struct ExpressionContext {
	request: Option<RequestContext>,
	response: Option<ResponseContext>,
	jwt: Option<Claims>,
	mcp: Option<crate::mcp::rbac::ResourceType>,
}

#[derive(Clone, Debug, Serialize)]
struct RequestContext {
	#[serde(with = "http_serde::method")]
	method: ::http::Method,

	#[serde(with = "http_serde::uri")]
	uri: ::http::Uri,

	#[serde(with = "http_serde::header_map")]
	headers: ::http::HeaderMap,
}

#[derive(Clone, Debug, Serialize)]
struct ResponseContext {
	#[serde(with = "http_serde::status_code")]
	code: ::http::StatusCode,
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

#[cfg(any(test, feature = "internal_benches"))]
pub mod tests {
	use super::*;
	use crate::http::Body;
	use crate::store::Stores;
	use crate::types::agent::{Listener, ListenerProtocol, PathMatch, Route, RouteMatch, RouteSet};
	use agent_core::strng;
	use divan::Bencher;
	use http::Method;
	use std::net::{IpAddr, Ipv4Addr, SocketAddr};

	#[test]
	fn expression() {
		let mut expr =
			ExpressionCall::new(r#"request.method == "GET" && request.headers["x-example"] == "value""#)
				.unwrap();
		let req = ::http::Request::builder()
			.method(Method::GET)
			.uri("http://example.com")
			.header("x-example", "value")
			.body(Body::empty())
			.unwrap();
		expr.with_request(&req);
		assert_eq!(Value::Bool(true), expr.eval().unwrap());
	}

	#[divan::bench]
	fn bench_with_request(b: Bencher) {
		let expr = Arc::new(
			Expression::new(r#"request.method == "GET" && request.headers["x-example"] == "value""#)
				.unwrap(),
		);
		b.with_inputs(|| {
			::http::Request::builder()
				.method(Method::GET)
				.uri("http://example.com")
				.header("x-example", "value")
				.body(Body::empty())
				.unwrap()
		})
		.bench_refs(|r| {
			let mut ec = ExpressionCall::from_expression(expr.clone());
			ec.with_request(r);
			ec.eval().unwrap();
		});
	}

	#[divan::bench]
	fn bench(b: Bencher) {
		let expr = Arc::new(Expression::new(r#"1 + 2 == 3"#).unwrap());
		b.with_inputs(|| {
			::http::Request::builder()
				.method(Method::GET)
				.uri("http://example.com")
				.header("x-example", "value")
				.body(Body::empty())
				.unwrap()
		})
		.bench_refs(|r| {
			let mut ec = ExpressionCall::from_expression(expr.clone());
			ec.with_request(r);
			ec.eval().unwrap();
		});
	}
}
