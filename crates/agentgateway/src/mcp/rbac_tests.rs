#[cfg(test)]
use assert_matches::assert_matches;
use divan::Bencher;
use secrecy::SecretString;
use serde_json::{Map, Value};

use super::*;

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
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("sub".to_string(), "1234567890".to_string().into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec), Ok(false));
}

#[test]
fn test_rbac_check_exact_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.sub == "1234567890""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("sub".to_string(), "1234567890".to_string().into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec), Ok(true));
}

#[test]
fn test_rbac_target() {
	let policies = vec![r#"mcp.tool.name == "increment" && mcp.tool.target == "server""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("sub".to_string(), "1234567890".to_string().into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec), Ok(true));

	let exec_different_target = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"not-server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec_different_target), Ok(false));
}

#[test]
fn test_rbac_check_contains_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.groups == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("groups".to_string(), "admin".to_string().into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec), Ok(true));
}

#[test]
fn test_rbac_check_nested_key_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.user.role == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	let mut user_obj = Map::new();
	user_obj.insert("role".to_string(), "admin".into());
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("user".to_string(), user_obj.into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec), Ok(true));
}

#[test]
fn test_rbac_check_array_contains_match() {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.roles.contains("admin")"#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	let roles: Vec<Value> = vec!["user".into(), "admin".into(), "developer".into()];
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("roles".to_string(), roles.into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();

	assert_matches!(rbac.validate_internal(&exec), Ok(true));
}

#[divan::bench]
fn bench(b: Bencher) {
	let policies = vec![r#"mcp.tool.name == "increment" && jwt.user.role == "admin""#];
	let rbac = RuleSet::new(create_policy_set(policies));
	let mut ctx = ContextBuilder::new();
	RuleSets::from(vec![rbac.clone()]).register(&mut ctx);
	let mut user_obj = Map::new();
	user_obj.insert("role".to_string(), "admin".into());
	ctx.with_jwt(&Claims {
		inner: Map::from_iter([("user".to_string(), user_obj.into())]),
		jwt: SecretString::new("".into()),
	});
	let exec = ctx
		.build_with_mcp(Some(&ResourceType::Tool(ResourceId::new(
			"server".to_string(),
			"increment".to_string(),
		))))
		.unwrap();
	b.bench(|| {
		rbac.validate_internal(&exec);
	});
}
