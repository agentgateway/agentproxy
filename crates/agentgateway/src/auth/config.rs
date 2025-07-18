use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::serdes::FileInlineOrRemote;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AuthConfig {
    pub enabled: bool,
    pub web_interface_address: Option<String>,
    pub session: SessionConfig,
    pub users: UserConfig,
    pub oauth2: OAuth2Config,
    pub security: SecurityConfig,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            web_interface_address: None,
            session: SessionConfig::default(),
            users: UserConfig::default(),
            oauth2: OAuth2Config::default(),
            security: SecurityConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SessionConfig {
    pub cookie_name: String,
    pub max_age: Duration,
    pub secure: bool,
    pub same_site: SameSitePolicy,
    pub http_only: bool,
    pub domain: Option<String>,
    pub path: String,
    pub secret_key: String,
    pub refresh_threshold: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: "agentgateway_session".to_string(),
            max_age: Duration::from_secs(8 * 60 * 60), // 8 hours
            secure: true,
            same_site: SameSitePolicy::Lax,
            http_only: true,
            domain: None,
            path: "/".to_string(),
            secret_key: "change-me-in-production".to_string(),
            refresh_threshold: Duration::from_secs(30 * 60), // 30 minutes
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum UserConfig {
    File {
        users: Vec<FileUser>,
    },
    Database {
        connection_string: String,
    },
}

impl Default for UserConfig {
    fn default() -> Self {
        Self::File {
            users: vec![FileUser {
                username: "admin".to_string(),
                password_hash: "$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LeqZnwHfJpFZqEUhm".to_string(), // "admin"
                email: Some("admin@example.com".to_string()),
                roles: vec!["admin".to_string()],
                enabled: true,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FileUser {
    pub username: String,
    #[serde(serialize_with = "crate::serdes::ser_redact")]
    pub password_hash: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct OAuth2Config {
    pub enabled: bool,
    pub providers: Vec<OAuth2ProviderConfig>,
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self {
            enabled: false,
            providers: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct OAuth2ProviderConfig {
    pub name: String,
    pub display_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub discovery_url: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub userinfo_url: Option<String>,
    pub jwks_url: Option<String>,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub user_mapping: UserMapping,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct UserMapping {
    pub username_field: String,
    pub email_field: Option<String>,
    pub name_field: Option<String>,
    pub groups_field: Option<String>,
    pub role_mapping: std::collections::HashMap<String, Vec<String>>,
}

impl Default for UserMapping {
    fn default() -> Self {
        Self {
            username_field: "sub".to_string(),
            email_field: Some("email".to_string()),
            name_field: Some("name".to_string()),
            groups_field: Some("groups".to_string()),
            role_mapping: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SecurityConfig {
    pub password_policy: PasswordPolicy,
    pub rate_limiting: RateLimitConfig,
    pub csrf_protection: bool,
    pub require_https: bool,
    pub trusted_proxies: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            password_policy: PasswordPolicy::default(),
            rate_limiting: RateLimitConfig::default(),
            csrf_protection: true,
            require_https: true,
            trusted_proxies: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
    pub max_age_days: Option<u32>,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_symbols: false,
            max_age_days: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub max_attempts: u32,
    pub window_minutes: u32,
    pub lockout_minutes: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 5,
            window_minutes: 15,
            lockout_minutes: 30,
        }
    }
}
