use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::auth::{AuthError, FileUser, OAuth2ProviderConfig, UserMapping};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub enabled: bool,
    pub password_hash: Option<String>,
    pub created_at: std::time::SystemTime,
    pub last_login: Option<std::time::SystemTime>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2UserInfo {
    pub sub: String,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub groups: Vec<String>,
    pub raw_claims: HashMap<String, serde_json::Value>,
}

#[async_trait]
pub trait UserProvider: Send + Sync {
    async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError>;
    async fn get_user_by_id(&self, user_id: &str) -> Result<User, AuthError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, AuthError>;
    async fn update_last_login(&self, user_id: &str) -> Result<(), AuthError>;
    async fn is_user_enabled(&self, user_id: &str) -> Result<bool, AuthError>;
}

pub struct FileUserProvider {
    users: Vec<FileUser>,
}

impl FileUserProvider {
    pub fn new(users: Vec<FileUser>) -> Self {
        Self { users }
    }

    fn file_user_to_user(&self, file_user: &FileUser) -> User {
        User {
            id: file_user.username.clone(),
            username: file_user.username.clone(),
            email: file_user.email.clone(),
            roles: file_user.roles.clone(),
            enabled: file_user.enabled,
            password_hash: Some(file_user.password_hash.clone()),
            created_at: std::time::SystemTime::now(),
            last_login: None,
            metadata: HashMap::new(),
        }
    }
}

#[async_trait]
impl UserProvider for FileUserProvider {
    async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AuthError> {
        let file_user = self.users.iter()
            .find(|u| u.username == username && u.enabled)
            .ok_or(AuthError::InvalidCredentials)?;

        if verify(password, &file_user.password_hash)? {
            Ok(self.file_user_to_user(file_user))
        } else {
            Err(AuthError::InvalidCredentials)
        }
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User, AuthError> {
        let file_user = self.users.iter()
            .find(|u| u.username == user_id)
            .ok_or(AuthError::UserNotFound)?;

        Ok(self.file_user_to_user(file_user))
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, AuthError> {
        self.get_user_by_id(username).await
    }

    async fn update_last_login(&self, _user_id: &str) -> Result<(), AuthError> {
        // File-based provider doesn't persist login times
        Ok(())
    }

    async fn is_user_enabled(&self, user_id: &str) -> Result<bool, AuthError> {
        let file_user = self.users.iter()
            .find(|u| u.username == user_id)
            .ok_or(AuthError::UserNotFound)?;

        Ok(file_user.enabled)
    }
}

pub struct DatabaseUserProvider {
    _connection_string: String,
}

impl DatabaseUserProvider {
    pub async fn new(_connection_string: String) -> Result<Self, AuthError> {
        // TODO: Implement database connection
        Ok(Self {
            _connection_string,
        })
    }
}

#[async_trait]
impl UserProvider for DatabaseUserProvider {
    async fn authenticate_user(&self, _username: &str, _password: &str) -> Result<User, AuthError> {
        // TODO: Implement database authentication
        Err(AuthError::InternalError("Database provider not implemented".to_string()))
    }

    async fn get_user_by_id(&self, _user_id: &str) -> Result<User, AuthError> {
        // TODO: Implement database lookup
        Err(AuthError::InternalError("Database provider not implemented".to_string()))
    }

    async fn get_user_by_username(&self, _username: &str) -> Result<User, AuthError> {
        // TODO: Implement database lookup
        Err(AuthError::InternalError("Database provider not implemented".to_string()))
    }

    async fn update_last_login(&self, _user_id: &str) -> Result<(), AuthError> {
        // TODO: Implement database update
        Ok(())
    }

    async fn is_user_enabled(&self, _user_id: &str) -> Result<bool, AuthError> {
        // TODO: Implement database check
        Ok(true)
    }
}

#[async_trait]
pub trait OAuth2Provider: Send + Sync {
    fn name(&self) -> &str;
    fn display_name(&self) -> &str;
    fn get_authorization_url(&self, state: &str) -> Result<String, AuthError>;
    async fn exchange_code(&self, code: &str, state: &str) -> Result<OAuth2Tokens, AuthError>;
    async fn get_user_info(&self, access_token: &str) -> Result<OAuth2UserInfo, AuthError>;
    fn map_user_info(&self, user_info: &OAuth2UserInfo) -> User;
}

#[derive(Debug, Clone)]
pub struct OAuth2Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
}

pub struct OAuth2ProviderImpl {
    config: OAuth2ProviderConfig,
    client: reqwest::Client,
    discovery_info: Option<OpenIDConnectDiscovery>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenIDConnectDiscovery {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: Option<String>,
    jwks_uri: String,
    scopes_supported: Option<Vec<String>>,
    response_types_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
}

impl OAuth2ProviderImpl {
    pub async fn new(config: OAuth2ProviderConfig, client: reqwest::Client) -> Result<Self, AuthError> {
        let discovery_info = if let Some(discovery_url) = &config.discovery_url {
            Some(Self::fetch_discovery_info(&client, discovery_url).await?)
        } else {
            None
        };

        Ok(Self {
            config,
            client,
            discovery_info,
        })
    }

    async fn fetch_discovery_info(client: &reqwest::Client, discovery_url: &str) -> Result<OpenIDConnectDiscovery, AuthError> {
        let response = client
            .get(discovery_url)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        let discovery_info: OpenIDConnectDiscovery = response
            .json()
            .await
            .map_err(|e| AuthError::SerializationError(e.to_string()))?;

        Ok(discovery_info)
    }

    fn get_auth_url(&self) -> &str {
        if let Some(discovery) = &self.discovery_info {
            &discovery.authorization_endpoint
        } else {
            self.config.auth_url.as_ref().unwrap()
        }
    }

    fn get_token_url(&self) -> &str {
        if let Some(discovery) = &self.discovery_info {
            &discovery.token_endpoint
        } else {
            self.config.token_url.as_ref().unwrap()
        }
    }

    fn get_userinfo_url(&self) -> Option<&str> {
        if let Some(discovery) = &self.discovery_info {
            discovery.userinfo_endpoint.as_deref()
        } else {
            self.config.userinfo_url.as_deref()
        }
    }
}

#[async_trait]
impl OAuth2Provider for OAuth2ProviderImpl {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn display_name(&self) -> &str {
        &self.config.display_name
    }

    fn get_authorization_url(&self, state: &str) -> Result<String, AuthError> {
        let mut url = url::Url::parse(self.get_auth_url())
            .map_err(|e| AuthError::ConfigError(format!("Invalid auth URL: {}", e)))?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.config.client_id)
            .append_pair("redirect_uri", &self.config.redirect_uri)
            .append_pair("scope", &self.config.scopes.join(" "))
            .append_pair("state", state);

        Ok(url.to_string())
    }

    async fn exchange_code(&self, code: &str, _state: &str) -> Result<OAuth2Tokens, AuthError> {
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("redirect_uri", &self.config.redirect_uri);
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);

        let response = self.client
            .post(self.get_token_url())
            .form(&params)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AuthError::OAuth2Error(format!("Token exchange failed: {}", error_text)));
        }

        let token_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::SerializationError(e.to_string()))?;

        let access_token = token_response["access_token"]
            .as_str()
            .ok_or_else(|| AuthError::OAuth2Error("Missing access_token in response".to_string()))?
            .to_string();

        let refresh_token = token_response["refresh_token"]
            .as_str()
            .map(|s| s.to_string());

        let id_token = token_response["id_token"]
            .as_str()
            .map(|s| s.to_string());

        let token_type = token_response["token_type"]
            .as_str()
            .unwrap_or("Bearer")
            .to_string();

        let expires_in = token_response["expires_in"]
            .as_u64();

        let scope = token_response["scope"]
            .as_str()
            .map(|s| s.to_string());

        Ok(OAuth2Tokens {
            access_token,
            refresh_token,
            id_token,
            token_type,
            expires_in,
            scope,
        })
    }

    async fn get_user_info(&self, access_token: &str) -> Result<OAuth2UserInfo, AuthError> {
        let userinfo_url = self.get_userinfo_url()
            .ok_or_else(|| AuthError::ConfigError("No userinfo endpoint configured".to_string()))?;

        let response = self.client
            .get(userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AuthError::OAuth2Error(format!("Failed to get user info: {}", error_text)));
        }

        let user_info: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::SerializationError(e.to_string()))?;

        let sub = user_info["sub"]
            .as_str()
            .ok_or_else(|| AuthError::OAuth2Error("Missing 'sub' claim in user info".to_string()))?
            .to_string();

        let username = user_info[&self.config.user_mapping.username_field]
            .as_str()
            .unwrap_or(&sub)
            .to_string();

        let email = if let Some(email_field) = &self.config.user_mapping.email_field {
            user_info[email_field].as_str().map(|s| s.to_string())
        } else {
            None
        };

        let name = if let Some(name_field) = &self.config.user_mapping.name_field {
            user_info[name_field].as_str().map(|s| s.to_string())
        } else {
            None
        };

        let groups = if let Some(groups_field) = &self.config.user_mapping.groups_field {
            user_info[groups_field]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let raw_claims = user_info.as_object()
            .unwrap_or(&serde_json::Map::new())
            .clone()
            .into_iter()
            .collect::<HashMap<String, serde_json::Value>>();

        Ok(OAuth2UserInfo {
            sub,
            username,
            email,
            name,
            groups,
            raw_claims,
        })
    }

    fn map_user_info(&self, user_info: &OAuth2UserInfo) -> User {
        // Map groups to roles based on configuration
        let mut roles = Vec::new();
        for group in &user_info.groups {
            if let Some(mapped_roles) = self.config.user_mapping.role_mapping.get(group) {
                roles.extend(mapped_roles.clone());
            }
        }

        // If no roles were mapped, assign default user role
        if roles.is_empty() {
            roles.push("user".to_string());
        }

        User {
            id: user_info.sub.clone(),
            username: user_info.username.clone(),
            email: user_info.email.clone(),
            roles,
            enabled: true,
            password_hash: None,
            created_at: std::time::SystemTime::now(),
            last_login: None,
            metadata: user_info.raw_claims.iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    hash(password, DEFAULT_COST).map_err(AuthError::from)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    verify(password, hash).map_err(AuthError::from)
}
