use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Invalid session")]
    InvalidSession,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User disabled")]
    UserDisabled,
    
    #[error("Account locked")]
    AccountLocked,
    
    #[error("Rate limit exceeded")]
    RateLimited,
    
    #[error("CSRF token missing or invalid")]
    CsrfError,
    
    #[error("OAuth2 error: {0}")]
    OAuth2Error(String),
    
    #[error("JWT error: {0}")]
    JwtError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<serde_json::Error> for AuthError {
    fn from(err: serde_json::Error) -> Self {
        AuthError::SerializationError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AuthError::JwtError(err.to_string())
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(err: reqwest::Error) -> Self {
        AuthError::NetworkError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for AuthError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AuthError::CryptoError(err.to_string())
    }
}

impl AuthError {
    pub fn is_authentication_failure(&self) -> bool {
        matches!(
            self,
            AuthError::InvalidCredentials
                | AuthError::SessionExpired
                | AuthError::SessionNotFound
                | AuthError::InvalidSession
                | AuthError::UserNotFound
                | AuthError::UserDisabled
                | AuthError::AccountLocked
        )
    }
    
    pub fn status_code(&self) -> u16 {
        match self {
            AuthError::InvalidCredentials
            | AuthError::SessionExpired
            | AuthError::SessionNotFound
            | AuthError::InvalidSession
            | AuthError::UserNotFound
            | AuthError::UserDisabled
            | AuthError::AccountLocked => 401,
            AuthError::RateLimited => 429,
            AuthError::CsrfError => 403,
            _ => 500,
        }
    }
}
