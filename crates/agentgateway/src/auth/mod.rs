pub mod config;
pub mod handlers;
pub mod middleware;
pub mod session;
pub mod providers;
pub mod error;

pub use config::*;
pub use handlers::*;
pub use middleware::*;
pub use session::*;
pub use providers::*;
pub use error::*;

use std::sync::Arc;
use crate::client::Client;

/// Main authentication service that coordinates all auth functionality
#[derive(Clone)]
pub struct AuthService {
    pub session_manager: Arc<SessionManager>,
    pub user_provider: Arc<dyn UserProvider>,
    pub oauth2_providers: Vec<Arc<dyn OAuth2Provider>>,
    pub config: AuthConfig,
}

impl AuthService {
    pub async fn new(config: AuthConfig, client: Client) -> Result<Self, AuthError> {
        let session_manager = Arc::new(SessionManager::new(config.session.clone())?);
        
        // Initialize user provider based on config
        let user_provider: Arc<dyn UserProvider> = match &config.users {
            UserConfig::File { users } => Arc::new(FileUserProvider::new(users.clone())),
            UserConfig::Database { connection_string } => {
                Arc::new(DatabaseUserProvider::new(connection_string.clone()).await?)
            }
        };

        // Initialize OAuth2 providers
        let mut oauth2_providers = Vec::new();
        for provider_config in &config.oauth2.providers {
            let provider = OAuth2ProviderImpl::new(provider_config.clone(), reqwest::Client::new()).await?;
            oauth2_providers.push(Arc::new(provider) as Arc<dyn OAuth2Provider>);
        }

        Ok(Self {
            session_manager,
            user_provider,
            oauth2_providers,
            config,
        })
    }

    pub fn get_oauth2_provider(&self, name: &str) -> Option<&Arc<dyn OAuth2Provider>> {
        self.oauth2_providers.iter().find(|p| p.name() == name)
    }
}
