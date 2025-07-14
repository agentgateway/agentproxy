use crate::http::jwt::Claims;
use anyhow::{Context as _, Error};

use lazy_static::lazy_static;

use crate::*;
use secrecy::SecretString;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_json::map::Map;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;
use tracing::log;
use x509_parser::asn1_rs::AsTaggedExplicit;
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RuleSet {
	#[serde(serialize_with = "se_policies", deserialize_with = "de_policies")]
	#[cfg_attr(feature = "schema", schemars(with = "Vec<String>"))]
	pub rules: PolicySet,
}

#[derive(Clone, Debug)]
pub struct PolicySet(Vec<Arc<cel::Expression>>);

impl Default for PolicySet {
	fn default() -> Self {
		Self::new()
	}
}

impl PolicySet {
	pub fn new() -> Self {
		Self(Vec::new())
	}
	pub fn add(&mut self, p: impl Into<String>) -> Result<(), cel::Error> {
		self.0.push(Arc::new(cel::Expression::new(p)?));
		Ok(())
	}
}

pub fn se_policies<S: Serializer>(t: &PolicySet, serializer: S) -> Result<S::Ok, S::Error> {
	let mut seq = serializer.serialize_seq(Some(t.0.len()))?;
	for tt in &t.0 {
		seq.serialize_element(&format!("{tt:?}"))?;
	}
	seq.end()
}

pub fn de_policies<'de: 'a, 'a, D>(deserializer: D) -> Result<PolicySet, D::Error>
where
	D: Deserializer<'de>,
{
	let raw = Vec::<String>::deserialize(deserializer)?;
	let parsed: Vec<_> = raw
		.into_iter()
		.map(|r| cel::Expression::new(r).map(Arc::new))
		.collect::<Result<_, _>>()
		.map_err(|e| serde::de::Error::custom(e.to_string()))?;
	Ok(PolicySet(parsed))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RuleSets(Vec<RuleSet>);

impl From<Vec<RuleSet>> for RuleSets {
	fn from(value: Vec<RuleSet>) -> Self {
		Self(value)
	}
}

impl RuleSets {
	pub fn validate(&self, resource: &ResourceType, claims: &Identity) -> bool {
		// If there are no rule sets, everyone has access
		if self.0.is_empty() {
			return true;
		}
		self
			.0
			.iter()
			.any(|rule_set| rule_set.validate(resource, claims))
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl RuleSet {
	pub fn new(rules: PolicySet) -> Self {
		Self { rules }
	}

	// Check if the claims have access to the resource
	pub fn validate(&self, resource: &ResourceType, claims: &Identity) -> bool {
		self
			.validate_internal(resource, claims)
			.unwrap_or_else(|e| {
				tracing::warn!("authorization failed with error: {e}");
				// Fail closed
				false
			})
	}

	fn validate_internal(&self, resource: &ResourceType, claims: &Identity) -> anyhow::Result<bool> {
		tracing::debug!("Checking RBAC for resource: {:?}", resource);

		// If there are no rules, everyone has access
		if self.rules.0.is_empty() {
			return Ok(true);
		}

		for rule in &self.rules.0 {
			let mut exp = cel::ExpressionCall::from_expression(rule.clone());
			if let Some(claims) = claims.claims.as_ref() {
				exp.with_jwt(claims)
			}
			exp.with_mcp(resource);
			if exp.eval_bool() {
				return Ok(true);
			}
		}
		Ok(false)
	}
}

fn default_key_delimiter() -> String {
	".".to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ResourceType {
	Tool(ResourceId),
	Prompt(ResourceId),
	Resource(ResourceId),
}

impl ResourceType {
	fn target(&self) -> &str {
		match self {
			ResourceType::Tool(r) => &r.target,
			ResourceType::Prompt(r) => &r.target,
			ResourceType::Resource(r) => &r.target,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ResourceId {
	#[serde(default)]
	target: String,
	#[serde(rename = "name", default)]
	id: String,
}

impl ResourceId {
	pub fn new(target: String, id: String) -> Self {
		Self { target, id }
	}
}

#[derive(Clone, Debug, Default)]
pub struct Identity {
	pub claims: Option<Claims>,
	pub connection_id: Option<String>,
}

impl agent_core::trcng::Claim for Identity {
	fn get_claim(&self, key: &str) -> Option<&str> {
		self.get_claim(key, ".")
	}
}

impl Identity {
	pub fn empty() -> Self {
		Self {
			claims: None,
			connection_id: None,
		}
	}

	pub fn new(claims: Option<Claims>, connection_id: Option<String>) -> Self {
		Self {
			claims,
			connection_id,
		}
	}
	// Attempts to get the claim from the claims map
	// The key should be split by the key_delimiter and then the map should be searched recursively
	// If the key is not found, it returns None
	// If the key is found, it returns the value
	pub fn get_claim(&self, key: &str, key_delimiter: &str) -> Option<&str> {
		match &self.claims {
			Some(claims) => {
				// Split the key by the delimiter to handle nested lookups
				let keys = key.split(key_delimiter).collect::<Vec<&str>>();

				// Start with the root claims map
				let mut current_value = &claims.inner;

				// Navigate through each key level
				let num_keys = keys.len();
				for (index, key_part) in keys.into_iter().enumerate() {
					// Get the value at this level
					let value = current_value.get(key_part)?;

					// If this is the last key part, return the string value
					if index == num_keys - 1 {
						return value.as_str();
					}

					// Otherwise, try to navigate deeper if it's an object
					current_value = value.as_object()?;
				}

				None
			},
			None => None,
		}
	}
}

#[cfg(any(test, feature = "internal_benches"))]
#[path = "rbac_tests.rs"]
mod tests;
