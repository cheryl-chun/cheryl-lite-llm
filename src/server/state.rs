use crate::{database::{AuthRepository, DatabaseContext, DatabaseFactory}, router::Router};
use crate::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub router: Arc<Router>,
    pub db: DatabaseContext,
}

impl AppState {
    pub fn new(router: Router, db: DatabaseContext) -> Self {
        Self {
            router: Arc::new(router),
            db,
        }
    }

    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        let db = DatabaseFactory::create(&config.database).await?;
        tracing::info!("Database connection established");
    
        let router = Router::from_config(config)?;
        tracing::info!("Router initialized with {} providers", config.providers.len());

        Ok(Self::new(router, db))
    }
}
