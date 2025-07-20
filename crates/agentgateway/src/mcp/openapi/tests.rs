use std::borrow::Cow;
use std::sync::Arc;

use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use rmcp::model::Tool;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::*;
use crate::client::Client;

// Helper to create a handler and mock server for tests
async fn setup() -> (MockServer, Handler) {
	let server = MockServer::start().await;
	let host = server.uri();
	let parsed = reqwest::Url::parse(&host).unwrap();
	let client = Client::new(
		&client::Config {
			resolver_cfg: ResolverConfig::default(),
			resolver_opts: ResolverOpts::default(),
		},
		None,
	);

	// Define a sample tool for testing
	let test_tool_get = Tool {
		name: Cow::Borrowed("get_user"),
		description: Some(Cow::Borrowed("Get user details")), // Added description
		input_schema: Arc::new(
			json!({ // Define a simple schema for testing
					"type": "object",
					"properties": {
							"path": {
									"type": "object",
									"properties": {
											"user_id": {"type": "string"}
									},
									"required": ["user_id"]
							},
							"query": {
									"type": "object",
									"properties": {
											"verbose": {"type": "string"}
									}
							},
							"header": {
									"type": "object",
									"properties": {
											"X-Request-ID": {"type": "string"}
									}
							}
					},
					"required": ["path"] // Only path is required for this tool
			})
			.as_object()
			.unwrap()
			.clone(),
		),
		annotations: None,
	};
	let upstream_call_get = UpstreamOpenAPICall {
		method: "GET".to_string(),
		path: "/users/{user_id}".to_string(),
	};

	let test_tool_post = Tool {
		name: Cow::Borrowed("create_user"),
		description: Some(Cow::Borrowed("Create a new user")),
		input_schema: Arc::new(
			json!({
				"type": "object",
				"properties": {
					"body": {
						"type": "object",
						"properties": {
							"name": {"type": "string"},
							"email": {"type": "string"}
						},
						"required": ["name", "email"]
					},
					"query": {
						"type": "object",
						"properties": {
							"source": {"type": "string"}
						}
					},
					"header": {
						"type": "object",
						"properties": {
							"X-API-Key": {"type": "string"}
						}
					}
				},
				"required": ["body"]
			})
			.as_object()
			.unwrap()
			.clone(),
		),
		annotations: None,
	};
	let upstream_call_post = UpstreamOpenAPICall {
		method: "POST".to_string(),
		path: "/users".to_string(),
	};

	let handler = Handler {
		host: parsed.host().unwrap().to_string(),
		prefix: "".to_string(),
		port: parsed.port().unwrap_or(8080),
		client,
		tools: vec![
			(test_tool_get, upstream_call_get),
			(test_tool_post, upstream_call_post),
		],
		policies: BackendPolicies::default(),
	};

	(server, handler)
}

#[tokio::test]
async fn test_call_tool_get_simple_success() {
	let (server, handler) = setup().await;

	let user_id = "123";
	let expected_response = json!({ "id": user_id, "name": "Test User" });

	Mock::given(method("GET"))
		.and(path(format!("/users/{user_id}")))
		.respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let args = json!({ "path": { "user_id": user_id } });
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}

#[tokio::test]
async fn test_call_tool_get_with_query() {
	let (server, handler) = setup().await;

	let user_id = "456";
	let verbose_flag = "true";
	let expected_response =
		json!({ "id": user_id, "name": "Test User", "details": "Verbose details" });

	Mock::given(method("GET"))
		.and(path(format!("/users/{user_id}")))
		.and(query_param("verbose", verbose_flag))
		.respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let args = json!({ "path": { "user_id": user_id }, "query": { "verbose": verbose_flag } });
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}

#[tokio::test]
async fn test_call_tool_get_with_header() {
	let (server, handler) = setup().await;

	let user_id = "789";
	let request_id = "req-abc";
	let expected_response = json!({ "id": user_id, "name": "Another User" });

	Mock::given(method("GET"))
		.and(path(format!("/users/{user_id}")))
		.and(header("X-Request-ID", request_id))
		.respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let args = json!({ "path": { "user_id": user_id }, "header": { "X-Request-ID": request_id } });
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}

#[tokio::test]
async fn test_call_tool_post_with_body() {
	let (server, handler) = setup().await;

	let request_body = json!({ "name": "New User", "email": "new@example.com" });
	let expected_response = json!({ "id": "xyz", "name": "New User", "email": "new@example.com" });

	Mock::given(method("POST"))
		.and(path("/users"))
		.and(body_json(&request_body))
		.respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let args = json!({ "body": request_body });
	let result = handler
		.call_tool("create_user", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}

#[tokio::test]
async fn test_call_tool_post_all_params() {
	let (server, handler) = setup().await;

	let request_body = json!({ "name": "Complete User", "email": "complete@example.com" });
	let api_key = "secret-key";
	let source = "test-suite";
	let expected_response = json!({ "id": "comp-123", "name": "Complete User" });

	Mock::given(method("POST"))
		.and(path("/users"))
		.and(query_param("source", source))
		.and(header("X-API-Key", api_key))
		.and(body_json(&request_body))
		.respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let args = json!({
			"body": request_body,
			"query": { "source": source },
			"header": { "X-API-Key": api_key }
	});
	let result = handler
		.call_tool("create_user", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}

#[tokio::test]
async fn test_call_tool_tool_not_found() {
	let (_server, handler) = setup().await; // Mock server not needed

	let args = json!({});
	let result = handler
		.call_tool("nonexistent_tool", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_err());
	assert!(
		result
			.unwrap_err()
			.to_string()
			.contains("tool nonexistent_tool not found")
	);
}

#[tokio::test]
async fn test_call_tool_upstream_error() {
	let (server, handler) = setup().await;

	let user_id = "error-user";
	let error_response = json!({ "error": "User not found" });

	Mock::given(method("GET"))
		.and(path(format!("/users/{user_id}")))
		.respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
		.mount(&server)
		.await;

	let args = json!({ "path": { "user_id": user_id } });
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;

	assert!(result.is_err());
	let err = result.unwrap_err();
	assert!(err.to_string().contains("failed with status 404 Not Found"));
	assert!(err.to_string().contains(&error_response.to_string()));
}

#[tokio::test]
async fn test_call_tool_invalid_header_value() {
	let (server, handler) = setup().await;

	let user_id = "header-issue";
	// Mock is set up but won't be hit because header construction fails client-side
	Mock::given(method("GET"))
		.and(path(format!("/users/{user_id}")))
		.respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": user_id })))
		.mount(&server)
		.await;

	// Intentionally provide a non-string header value
	let args = json!({
			"path": { "user_id": user_id },
			"header": { "X-Request-ID": 12345 } // Invalid header value (not a string)
	});

	// We expect the call to succeed, but the invalid header should be skipped (and logged)
	// The mock doesn't expect the header, so if the request goes through without it, it passes.
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;
	assert!(result.is_ok()); // Check that the call still succeeds despite the bad header
	assert_eq!(result.unwrap(), json!({ "id": user_id }).to_string());
	// We can't easily assert the log message here, but manual inspection of logs would show the warning.
}

#[tokio::test]
async fn test_call_tool_invalid_query_param_value() {
	let (server, handler) = setup().await;

	let user_id = "query-issue";
	// Mock is set up but won't be hit with the invalid query param
	Mock::given(method("GET"))
		.and(path(format!("/users/{user_id}")))
		// IMPORTANT: We don't .and(query_param(...)) here because the invalid param is skipped
		.respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": user_id })))
		.mount(&server)
		.await;

	// Intentionally provide a non-string query value
	let args = json!({
			"path": { "user_id": user_id },
			"query": { "verbose": true } // Invalid query value (not a string)
	});

	// We expect the call to succeed, but the invalid query param should be skipped (and logged)
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), json!({ "id": user_id }).to_string());
}

#[tokio::test]
async fn test_call_tool_invalid_path_param_value() {
	let (server, handler) = setup().await;

	let invalid_user_id = json!(12345); // Not a string
	// Mock is set up for the *literal* path, as substitution will fail
	Mock::given(method("GET"))
		.and(path("/users/{user_id}")) // Path doesn't get substituted
		.respond_with(
			ResponseTemplate::new(404) // Or whatever the server does with a literal {user_id}
				.set_body_string("Not Found - Literal Path"),
		)
		.mount(&server)
		.await;

	let args = json!({
			"path": { "user_id": invalid_user_id }
	});

	// The call might succeed at the HTTP level but might return an error from the server,
	// or potentially fail if the path is fundamentally invalid after non-substitution.
	// Here we assume the server returns 404 for the literal path.
	let result = handler
		.call_tool("get_user", Some(args.as_object().unwrap().clone()))
		.await;

	// Depending on server behavior for the literal path, this might be Ok or Err.
	// If server returns 404 for the literal path:
	assert!(result.is_err());
	assert!(
		result
			.unwrap_err()
			.to_string()
			.contains("failed with status 404 Not Found")
	);

	// If the request *itself* failed before sending (e.g., invalid URL formed),
	// the error might be different.
}

#[tokio::test]
async fn test_normalize_url_path_empty_prefix() {
	// Test the fix for double slash issue when prefix is empty (host/port config)
	let result = super::normalize_url_path("", "/mqtt/healthcheck");
	assert_eq!(result, "/mqtt/healthcheck");
}

#[tokio::test]
async fn test_normalize_url_path_with_prefix() {
	// Test with a prefix that has trailing slash
	let result = super::normalize_url_path("/api/v3/", "/pet");
	assert_eq!(result, "/api/v3/pet");
}

#[tokio::test]
async fn test_normalize_url_path_prefix_no_trailing_slash() {
	// Test with a prefix without trailing slash
	let result = super::normalize_url_path("/api/v3", "/pet");
	assert_eq!(result, "/api/v3/pet");
}

#[tokio::test]
async fn test_normalize_url_path_path_without_leading_slash() {
	// Test with path that doesn't start with slash
	let result = super::normalize_url_path("/api/v3", "pet");
	assert_eq!(result, "/api/v3/pet");
}

#[tokio::test]
async fn test_normalize_url_path_empty_prefix_path_without_slash() {
	// Test edge case: empty prefix and path without leading slash
	let result = super::normalize_url_path("", "pet");
	assert_eq!(result, "/pet");
}

#[tokio::test]
async fn test_call_tool_no_double_slash_with_empty_prefix() {
	// Test the actual fix in action - simulating host/port config scenario
	let server = MockServer::start().await;
	let host = server.uri();
	let parsed = reqwest::Url::parse(&host).unwrap();
	let client = Client::new(
		&client::Config {
			resolver_cfg: ResolverConfig::default(),
			resolver_opts: ResolverOpts::default(),
		},
		None,
	);

	let test_tool = Tool {
		name: Cow::Borrowed("mqtt_healthcheck"),
		description: Some(Cow::Borrowed("MQTT health check")),
		input_schema: Arc::new(
			json!({
				"type": "object",
				"properties": {},
				"required": []
			})
			.as_object()
			.unwrap()
			.clone(),
		),
		annotations: None,
	};
	let upstream_call = UpstreamOpenAPICall {
		method: "GET".to_string(),
		path: "/mqtt/healthcheck".to_string(),
	};

	// Handler with empty prefix (simulating host/port config)
	let handler = Handler {
		host: parsed.host().unwrap().to_string(),
		prefix: "".to_string(), // Empty prefix like when using host/port config
		port: parsed.port().unwrap_or(8080),
		client,
		tools: vec![(test_tool, upstream_call)],
		policies: BackendPolicies::default(),
	};

	let expected_response = json!({ "status": "healthy" });

	// Mock expects the path WITHOUT double slash
	Mock::given(method("GET"))
		.and(path("/mqtt/healthcheck")) // Single slash, not double
		.respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let result = handler
		.call_tool("mqtt_healthcheck", Some(serde_json::Map::new()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}

#[tokio::test]
async fn test_call_tool_with_server_prefix() {
	// Test that server prefix from OpenAPI servers section still works correctly
	let server = MockServer::start().await;
	let host = server.uri();
	let parsed = reqwest::Url::parse(&host).unwrap();
	let client = Client::new(
		&client::Config {
			resolver_cfg: ResolverConfig::default(),
			resolver_opts: ResolverOpts::default(),
		},
		None,
	);

	let test_tool = Tool {
		name: Cow::Borrowed("get_pet"),
		description: Some(Cow::Borrowed("Get pet by ID")),
		input_schema: Arc::new(
			json!({
				"type": "object",
				"properties": {},
				"required": []
			})
			.as_object()
			.unwrap()
			.clone(),
		),
		annotations: None,
	};
	let upstream_call = UpstreamOpenAPICall {
		method: "GET".to_string(),
		path: "/pet".to_string(),
	};

	// Handler with server prefix (simulating OpenAPI servers section)
	let handler = Handler {
		host: parsed.host().unwrap().to_string(),
		prefix: "/api/v3".to_string(), // Prefix from OpenAPI servers
		port: parsed.port().unwrap_or(8080),
		client,
		tools: vec![(test_tool, upstream_call)],
		policies: BackendPolicies::default(),
	};

	let expected_response = json!({ "id": 1, "name": "Fluffy" });

	// Mock expects the combined path
	Mock::given(method("GET"))
		.and(path("/api/v3/pet")) // Prefix + path, properly combined
		.respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
		.mount(&server)
		.await;

	let result = handler
		.call_tool("get_pet", Some(serde_json::Map::new()))
		.await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected_response.to_string());
}
