use super::*;
#[cfg(test)]
use assert_matches::assert_matches;
use divan::Bencher;
use secrecy::SecretString;
use serde_json::{Map, Value};

fn create_policy_set(policies: Vec<&str>) -> PolicySet {
	let mut policy_set = PolicySet::new();
	for p in policies.into_iter() {
		policy_set.add(p).expect("Failed to parse policy");
	}
	policy_set
}

#[test]
fn test_rbac_reject_exact_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.user == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	headers.insert("sub".to_string(), "1234567890".to_string().into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(false)
	);
}

#[test]
fn test_rbac_check_exact_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.sub == "1234567890""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	headers.insert("sub".to_string(), "1234567890".to_string().into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(true)
	);
}

#[test]
fn test_rbac_target() {
	let policies = vec![r#"mcp.tool.name == "increment" && mcp.tool.target == "server""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	headers.insert("sub".to_string(), "1234567890".to_string().into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(true)
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"not-server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(false)
	);
}

#[test]
fn test_rbac_check_contains_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.groups == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	// Use a simple string that matches exactly
	headers.insert("groups".to_string(), "admin".to_string().into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(true)
	);
}

#[test]
fn test_rbac_check_nested_key_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.user.role == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	let mut user_obj = Map::new();
	user_obj.insert("role".to_string(), "admin".into());
	headers.insert("user".to_string(), user_obj.into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(true)
	);
}

#[test]
fn test_rbac_check_array_contains_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.roles.contains("admin")"#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	// Create an array of roles
	let roles: Vec<Value> = vec!["user".into(), "admin".into(), "developer".into()];
	headers.insert("roles".to_string(), roles.into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	assert_matches!(
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string()
			)),
			&id
		),
		Ok(true)
	);
}

#[divan::bench]
fn bench(b: Bencher) {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.user.role == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut headers = Map::new();
	let mut user_obj = Map::new();
	user_obj.insert("role".to_string(), "admin".into());
	headers.insert("user".to_string(), user_obj.into());
	let id = Identity::new(
		Some(Claims {
			inner: headers,
			jwt: SecretString::new("".into()),
		}),
		None,
	);
	b.bench(|| {
		rbac.validate_internal(
			&ResourceType::Tool(ResourceId::new(
				"server".to_string(),
				"increment".to_string(),
			)),
			&id,
		);
	});
}
