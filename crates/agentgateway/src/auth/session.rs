use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use rand::Rng;
use crate::auth::{AuthError, SessionConfig, SameSitePolicy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub auth_method: AuthMethod,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub expires_at: SystemTime,
    pub csrf_token: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Traditional,
    OAuth2 { provider: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionClaims {
    sub: String,  // session_id
    iat: u64,     // issued at
    exp: u64,     // expires at
    user_id: String,
    username: String,
    email: Option<String>,
    roles: Vec<String>,
    auth_method: AuthMethod,
    csrf_token: String,
    metadata: HashMap<String, String>,
}

pub struct SessionManager {
    config: SessionConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl SessionManager {
    pub fn new(config: SessionConfig) -> Result<Self, AuthError> {
        let secret = config.secret_key.as_bytes();
        let encoding_key = EncodingKey::from_secret(secret);
        let decoding_key = DecodingKey::from_secret(secret);
        
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_exp = true;
        validation.leeway = 60; // 60 seconds leeway for clock skew
        
        Ok(Self {
            config,
            encoding_key,
            decoding_key,
            validation,
        })
    }

    pub fn create_session(&self, user_session: UserSession) -> Result<String, AuthError> {
        let now = SystemTime::now();
        let now_timestamp = now.duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| AuthError::InternalError(e.to_string()))?
            .as_secs();
        
        let exp_timestamp = user_session.expires_at.duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| AuthError::InternalError(e.to_string()))?
            .as_secs();

        let claims = SessionClaims {
            sub: user_session.session_id.clone(),
            iat: now_timestamp,
            exp: exp_timestamp,
            user_id: user_session.user_id,
            username: user_session.username,
            email: user_session.email,
            roles: user_session.roles,
            auth_method: user_session.auth_method,
            csrf_token: user_session.csrf_token,
            metadata: user_session.metadata,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn validate_session(&self, token: &str) -> Result<UserSession, AuthError> {
        let token_data = decode::<SessionClaims>(token, &self.decoding_key, &self.validation)?;
        let claims = token_data.claims;
        
        let created_at = SystemTime::UNIX_EPOCH + Duration::from_secs(claims.iat);
        let expires_at = SystemTime::UNIX_EPOCH + Duration::from_secs(claims.exp);
        
        // Check if session is expired
        if SystemTime::now() >= expires_at {
            return Err(AuthError::SessionExpired);
        }
        
        Ok(UserSession {
            session_id: claims.sub,
            user_id: claims.user_id,
            username: claims.username,
            email: claims.email,
            roles: claims.roles,
            auth_method: claims.auth_method,
            created_at,
            last_accessed: SystemTime::now(),
            expires_at,
            csrf_token: claims.csrf_token,
            metadata: claims.metadata,
        })
    }

    pub fn refresh_session(&self, session: &UserSession) -> Result<(UserSession, String), AuthError> {
        let now = SystemTime::now();
        let time_until_expiry = session.expires_at.duration_since(now)
            .map_err(|_| AuthError::SessionExpired)?;
        
        // Only refresh if we're within the refresh threshold
        if time_until_expiry > self.config.refresh_threshold {
            return Ok((session.clone(), String::new()));
        }
        
        let new_session = UserSession {
            session_id: session.session_id.clone(),
            user_id: session.user_id.clone(),
            username: session.username.clone(),
            email: session.email.clone(),
            roles: session.roles.clone(),
            auth_method: session.auth_method.clone(),
            created_at: session.created_at,
            last_accessed: now,
            expires_at: now + self.config.max_age,
            csrf_token: generate_csrf_token(),
            metadata: session.metadata.clone(),
        };
        
        let token = self.create_session(new_session.clone())?;
        Ok((new_session, token))
    }

    pub fn create_session_cookie(&self, token: &str) -> Cookie<'static> {
        let same_site = match self.config.same_site {
            SameSitePolicy::Strict => SameSite::Strict,
            SameSitePolicy::Lax => SameSite::Lax,
            SameSitePolicy::None => SameSite::None,
        };

        let mut cookie = Cookie::build((self.config.cookie_name.clone(), token.to_string()))
            .path(self.config.path.clone())
            .max_age(cookie::time::Duration::seconds(self.config.max_age.as_secs() as i64))
            .http_only(self.config.http_only)
            .secure(self.config.secure)
            .same_site(same_site);

        if let Some(domain) = &self.config.domain {
            cookie = cookie.domain(domain.clone());
        }

        cookie.build()
    }

    pub fn create_logout_cookie(&self) -> Cookie<'static> {
        let mut cookie = Cookie::build((self.config.cookie_name.clone(), ""))
            .path(self.config.path.clone())
            .max_age(cookie::time::Duration::seconds(0))
            .http_only(self.config.http_only)
            .secure(self.config.secure);

        if let Some(domain) = &self.config.domain {
            cookie = cookie.domain(domain.clone());
        }

        cookie.build()
    }

    pub fn generate_csrf_token(&self) -> String {
        generate_csrf_token()
    }

    pub fn validate_csrf_token(&self, session: &UserSession, provided_token: &str) -> bool {
        session.csrf_token == provided_token
    }
}

pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_csrf_token() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    (0..32)
        .map(|_| {
            let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            chars[rng.random_range(0..chars.len())] as char
        })
        .collect()
}

impl UserSession {
    pub fn new(
        user_id: String,
        username: String,
        email: Option<String>,
        roles: Vec<String>,
        auth_method: AuthMethod,
        session_duration: Duration,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            session_id: generate_session_id(),
            user_id,
            username,
            email,
            roles,
            auth_method,
            created_at: now,
            last_accessed: now,
            expires_at: now + session_duration,
            csrf_token: generate_csrf_token(),
            metadata: HashMap::new(),
        }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}
