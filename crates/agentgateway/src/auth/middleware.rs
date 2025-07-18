use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use tower_cookies::Cookies;
use crate::auth::{AuthService, AuthError, UserSession};

pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip authentication for certain paths
    let path = request.uri().path();
    if is_excluded_path(path) {
        return Ok(next.run(request).await);
    }

    // Check if authentication is enabled
    if !auth_service.config.enabled {
        return Ok(next.run(request).await);
    }

    // Get session cookie
    let session_cookie = cookies.get(&auth_service.config.session.cookie_name);
    
    let session = if let Some(cookie) = session_cookie {
        match auth_service.session_manager.validate_session(cookie.value()) {
            Ok(session) => {
                // Check if session needs refresh
                if let Ok((refreshed_session, new_token)) = auth_service.session_manager.refresh_session(&session) {
                    if !new_token.is_empty() {
                        let new_cookie = auth_service.session_manager.create_session_cookie(&new_token);
                        cookies.add(new_cookie);
                    }
                    Some(refreshed_session)
                } else {
                    Some(session)
                }
            }
            Err(AuthError::SessionExpired) => {
                // Remove expired session cookie
                let logout_cookie = auth_service.session_manager.create_logout_cookie();
                cookies.add(logout_cookie);
                None
            }
            Err(_) => None,
        }
    } else {
        None
    };

    match session {
        Some(session) => {
            // Add session to request extensions
            request.extensions_mut().insert(session);
            Ok(next.run(request).await)
        }
        None => {
            // Redirect to login for HTML requests, return 401 for API requests
            if is_api_path(path) {
                Err(StatusCode::UNAUTHORIZED)
            } else {
                // Return a redirect response
                let redirect_url = format!("/auth/login?redirect={}", urlencoding::encode(path));
                Ok(Response::builder()
                    .status(StatusCode::FOUND)
                    .header("Location", redirect_url)
                    .body(axum::body::Body::empty())
                    .unwrap())
            }
        }
    }
}

pub async fn csrf_middleware(
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Only check CSRF for state-changing requests
    if !is_state_changing_request(&request) {
        return Ok(next.run(request).await);
    }

    // Skip CSRF check if not enabled
    if !auth_service.config.security.csrf_protection {
        return Ok(next.run(request).await);
    }

    // Get session from request extensions
    let session = request.extensions().get::<UserSession>().cloned();
    
    if let Some(session) = session {
        // For JSON requests, check X-CSRF-Token header
        if let Some(csrf_header) = request.headers().get("X-CSRF-Token") {
            if let Ok(csrf_token) = csrf_header.to_str() {
                if auth_service.session_manager.validate_csrf_token(&session, csrf_token) {
                    return Ok(next.run(request).await);
                }
            }
        }
        
        // For form requests, check form data (would need to parse body)
        // This is a simplified implementation
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

pub fn role_required_middleware(
    required_roles: Vec<String>,
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let required_roles = required_roles.clone();
        Box::pin(async move {
            if let Some(session) = request.extensions().get::<UserSession>() {
                if required_roles.is_empty() || session.has_any_role(&required_roles.iter().map(|s| s.as_str()).collect::<Vec<_>>()) {
                    Ok(next.run(request).await)
                } else {
                    Err(StatusCode::FORBIDDEN)
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        })
    }
}

fn is_excluded_path(path: &str) -> bool {
    let excluded_paths = [
        "/auth/login",
        "/auth/logout",
        "/auth/oauth2",
        "/auth/health",
        "/health",
        "/metrics",
        "/debug",
        "/quitquitquit",
        "/config_dump",
        "/logging",
        "/assets",
        "/favicon.ico",
        "/robots.txt",
    ];

    excluded_paths.iter().any(|excluded| path.starts_with(excluded))
}

fn is_api_path(path: &str) -> bool {
    let api_paths = [
        "/api/",
        "/auth/",
        "/config",
        "/metrics",
    ];

    api_paths.iter().any(|api_path| path.starts_with(api_path))
}

fn is_state_changing_request(request: &Request) -> bool {
    matches!(request.method().as_str(), "POST" | "PUT" | "DELETE" | "PATCH")
}

// Extension trait for getting user session from request
pub trait RequestSessionExt {
    fn user_session(&self) -> Option<&UserSession>;
    fn require_user_session(&self) -> Result<&UserSession, AuthError>;
    fn require_role(&self, role: &str) -> Result<&UserSession, AuthError>;
    fn require_any_role(&self, roles: &[&str]) -> Result<&UserSession, AuthError>;
}

impl RequestSessionExt for Request {
    fn user_session(&self) -> Option<&UserSession> {
        self.extensions().get::<UserSession>()
    }

    fn require_user_session(&self) -> Result<&UserSession, AuthError> {
        self.user_session().ok_or(AuthError::SessionNotFound)
    }

    fn require_role(&self, role: &str) -> Result<&UserSession, AuthError> {
        let session = self.require_user_session()?;
        if session.has_role(role) {
            Ok(session)
        } else {
            Err(AuthError::InvalidCredentials)
        }
    }

    fn require_any_role(&self, roles: &[&str]) -> Result<&UserSession, AuthError> {
        let session = self.require_user_session()?;
        if session.has_any_role(roles) {
            Ok(session)
        } else {
            Err(AuthError::InvalidCredentials)
        }
    }
}
