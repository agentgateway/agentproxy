use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use axum::{
    extract::{Query, State, Form},
    response::{Html, Redirect, Response},
    http::{StatusCode, header},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookies, Cookie};
use uuid::Uuid;
use crate::auth::{AuthService, AuthError, UserSession, AuthMethod};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub csrf_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2CallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub csrf_token: Option<String>,
    pub redirect_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuth2ProvidersResponse {
    pub providers: Vec<OAuth2ProviderInfo>,
}

#[derive(Debug, Serialize)]
pub struct OAuth2ProviderInfo {
    pub name: String,
    pub display_name: String,
    pub auth_url: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub auth_method: AuthMethod,
    pub csrf_token: String,
}

pub async fn handle_login_page(
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
) -> Result<Html<String>, StatusCode> {
    // Check if user is already logged in
    if let Some(session_cookie) = cookies.get(&auth_service.config.session.cookie_name) {
        if auth_service.session_manager.validate_session(session_cookie.value()).is_ok() {
            return Ok(Html(r#"
                <script>window.location.href = '/';</script>
                <p>Redirecting...</p>
            "#.to_string()));
        }
    }

    let csrf_token = auth_service.session_manager.generate_csrf_token();
    let csrf_cookie = Cookie::build(("csrf_token", csrf_token.clone()))
        .path("/")
        .http_only(true)
        .secure(auth_service.config.session.secure)
        .max_age(cookie::time::Duration::minutes(10))
        .build();
    
    cookies.add(csrf_cookie);

    let oauth2_providers = auth_service.oauth2_providers.iter()
        .filter(|p| auth_service.config.oauth2.enabled)
        .map(|p| format!(
            r#"<a href="/auth/oauth2/{}/login" class="oauth2-provider">{}</a>"#,
            p.name(), p.display_name()
        ))
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Login - Agentgateway</title>
    <style>
        body {{ font-family: Arial, sans-serif; max-width: 400px; margin: 50px auto; padding: 20px; }}
        .form-group {{ margin-bottom: 15px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input[type="text"], input[type="password"] {{ width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; width: 100%; }}
        button:hover {{ background-color: #0056b3; }}
        .oauth2-provider {{ display: block; margin: 10px 0; padding: 10px; background-color: #f8f9fa; border: 1px solid #dee2e6; border-radius: 4px; text-decoration: none; color: #333; text-align: center; }}
        .oauth2-provider:hover {{ background-color: #e9ecef; }}
        .error {{ color: red; margin-bottom: 15px; }}
        .divider {{ margin: 20px 0; text-align: center; color: #666; }}
        .divider::before, .divider::after {{ content: ''; display: inline-block; width: 40%; height: 1px; background: #ccc; vertical-align: middle; }}
        .divider::before {{ margin-right: 10px; }}
        .divider::after {{ margin-left: 10px; }}
    </style>
</head>
<body>
    <h1>Login to Agentgateway</h1>
    <div id="error-message" class="error" style="display: none;"></div>
    
    {oauth2_section}
    
    <form id="login-form">
        <div class="form-group">
            <label for="username">Username:</label>
            <input type="text" id="username" name="username" required>
        </div>
        <div class="form-group">
            <label for="password">Password:</label>
            <input type="password" id="password" name="password" required>
        </div>
        <input type="hidden" name="csrf_token" value="{csrf_token}">
        <button type="submit">Login</button>
    </form>

    <script>
        document.getElementById('login-form').addEventListener('submit', async (e) => {{
            e.preventDefault();
            const formData = new FormData(e.target);
            const data = Object.fromEntries(formData);
            
            try {{
                const response = await fetch('/auth/login', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify(data)
                }});
                
                const result = await response.json();
                
                if (result.success) {{
                    window.location.href = result.redirect_url || '/';
                }} else {{
                    document.getElementById('error-message').textContent = result.message;
                    document.getElementById('error-message').style.display = 'block';
                }}
            }} catch (error) {{
                document.getElementById('error-message').textContent = 'Login failed. Please try again.';
                document.getElementById('error-message').style.display = 'block';
            }}
        }});
    </script>
</body>
</html>
"#,
        csrf_token = csrf_token,
        oauth2_section = if oauth2_providers.is_empty() {
            String::new()
        } else {
            format!(r#"
                <div class="oauth2-providers">
                    {}
                </div>
                <div class="divider">or</div>
            "#, oauth2_providers)
        }
    );

    Ok(Html(html))
}

pub async fn handle_login_submit(
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Validate CSRF token
    if let Some(csrf_token) = &login_req.csrf_token {
        if let Some(csrf_cookie) = cookies.get("csrf_token") {
            if csrf_cookie.value() != csrf_token {
                return Ok(Json(LoginResponse {
                    success: false,
                    message: "Invalid CSRF token".to_string(),
                    csrf_token: None,
                    redirect_url: None,
                }));
            }
        } else {
            return Ok(Json(LoginResponse {
                success: false,
                message: "CSRF token required".to_string(),
                csrf_token: None,
                redirect_url: None,
            }));
        }
    }

    // Authenticate user
    match auth_service.user_provider.authenticate_user(&login_req.username, &login_req.password).await {
        Ok(user) => {
            if !user.enabled {
                return Ok(Json(LoginResponse {
                    success: false,
                    message: "Account disabled".to_string(),
                    csrf_token: None,
                    redirect_url: None,
                }));
            }

            // Create session
            let user_id = user.id.clone();
            let session = UserSession::new(
                user_id.clone(),
                user.username,
                user.email,
                user.roles,
                AuthMethod::Traditional,
                auth_service.config.session.max_age,
            );

            // Generate session token
            let session_token = auth_service.session_manager.create_session(session.clone())
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Set session cookie
            let session_cookie = auth_service.session_manager.create_session_cookie(&session_token);
            cookies.add(session_cookie);

            // Remove CSRF cookie
            cookies.remove(Cookie::from("csrf_token"));

            // Update last login
            let _ = auth_service.user_provider.update_last_login(&user_id).await;

            Ok(Json(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
                csrf_token: Some(session.csrf_token),
                redirect_url: Some("/".to_string()),
            }))
        }
        Err(AuthError::InvalidCredentials) => {
            Ok(Json(LoginResponse {
                success: false,
                message: "Invalid username or password".to_string(),
                csrf_token: None,
                redirect_url: None,
            }))
        }
        Err(_) => {
            Ok(Json(LoginResponse {
                success: false,
                message: "Authentication failed".to_string(),
                csrf_token: None,
                redirect_url: None,
            }))
        }
    }
}

pub async fn handle_logout(
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
) -> Redirect {
    // Remove session cookie
    let logout_cookie = auth_service.session_manager.create_logout_cookie();
    cookies.add(logout_cookie);

    Redirect::to("/auth/login")
}

pub async fn handle_oauth2_providers(
    State(auth_service): State<Arc<AuthService>>,
) -> Json<OAuth2ProvidersResponse> {
    let providers = auth_service.oauth2_providers.iter()
        .filter(|_| auth_service.config.oauth2.enabled)
        .map(|p| {
            let state = Uuid::new_v4().to_string();
            let auth_url = p.get_authorization_url(&state).unwrap_or_default();
            
            OAuth2ProviderInfo {
                name: p.name().to_string(),
                display_name: p.display_name().to_string(),
                auth_url,
            }
        })
        .collect();

    Json(OAuth2ProvidersResponse { providers })
}

pub async fn handle_oauth2_login(
    State(auth_service): State<Arc<AuthService>>,
    axum::extract::Path(provider_name): axum::extract::Path<String>,
    cookies: Cookies,
) -> Result<Redirect, StatusCode> {
    let provider = auth_service.get_oauth2_provider(&provider_name)
        .ok_or(StatusCode::NOT_FOUND)?;

    let state = Uuid::new_v4().to_string();
    let auth_url = provider.get_authorization_url(&state)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store state in cookie for validation
    let state_cookie = Cookie::build(("oauth2_state", state.clone()))
        .path("/")
        .http_only(true)
        .secure(auth_service.config.session.secure)
        .max_age(cookie::time::Duration::minutes(10))
        .build();
    
    cookies.add(state_cookie);

    Ok(Redirect::to(&auth_url))
}

pub async fn handle_oauth2_callback(
    State(auth_service): State<Arc<AuthService>>,
    axum::extract::Path(provider_name): axum::extract::Path<String>,
    Query(params): Query<OAuth2CallbackQuery>,
    cookies: Cookies,
) -> Result<Redirect, StatusCode> {
    // Handle OAuth2 errors
    if let Some(error) = params.error {
        tracing::warn!("OAuth2 error: {} - {}", error, params.error_description.unwrap_or_default());
        return Ok(Redirect::to("/auth/login?error=oauth2_error"));
    }

    let code = params.code.ok_or(StatusCode::BAD_REQUEST)?;
    let state = params.state.ok_or(StatusCode::BAD_REQUEST)?;

    // Validate state
    if let Some(state_cookie) = cookies.get("oauth2_state") {
        if state_cookie.value() != state {
            return Ok(Redirect::to("/auth/login?error=invalid_state"));
        }
    } else {
        return Ok(Redirect::to("/auth/login?error=missing_state"));
    }

    let provider = auth_service.get_oauth2_provider(&provider_name)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Exchange code for tokens
    let tokens = provider.exchange_code(&code, &state).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get user info
    let user_info = provider.get_user_info(&tokens.access_token).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Map to internal user
    let user = provider.map_user_info(&user_info);

    // Create session
    let session = UserSession::new(
        user.id,
        user.username,
        user.email,
        user.roles,
        AuthMethod::OAuth2 { provider: provider_name },
        auth_service.config.session.max_age,
    );

    // Generate session token
    let session_token = auth_service.session_manager.create_session(session.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Set session cookie
    let session_cookie = auth_service.session_manager.create_session_cookie(&session_token);
    cookies.add(session_cookie);

    // Remove state cookie
    cookies.remove(Cookie::from("oauth2_state"));

    Ok(Redirect::to("/"))
}

pub async fn handle_user_info(
    State(auth_service): State<Arc<AuthService>>,
    cookies: Cookies,
) -> Result<Json<UserInfoResponse>, StatusCode> {
    let session_cookie = cookies.get(&auth_service.config.session.cookie_name)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let session = auth_service.session_manager.validate_session(session_cookie.value())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(UserInfoResponse {
        username: session.username,
        email: session.email,
        roles: session.roles,
        auth_method: session.auth_method,
        csrf_token: session.csrf_token,
    }))
}

pub async fn handle_health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}
