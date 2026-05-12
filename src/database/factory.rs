use std::{collections::HashMap, sync::{Arc, RwLock}};
use once_cell::sync::Lazy;
use anyhow::{Result, anyhow};

use crate::{config::DatabaseConfig, database::{DatabaseContext, builder::DatabaseBuilder}};

static REGISTRY: Lazy<RwLock<HashMap<String, Arc<dyn DatabaseBuilder>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
pub struct DatabaseFactory;

impl DatabaseFactory {
    pub fn register(db_type: &str, builder: Arc<dyn DatabaseBuilder>) {
        let mut registry = REGISTRY.write().unwrap();
        registry.insert(db_type.to_string(), builder);
    }

    pub async fn create(config: &DatabaseConfig) -> Result<DatabaseContext> {
        let db_type = Self::parse_db_type(&config.url)?;
        let registry = REGISTRY.read().unwrap();
        let builder = registry.get(&db_type)
        .ok_or_else(|| anyhow!(
            "Unsupported database type: {}, Available: {:?}",
            db_type,
            registry.keys().collect::<Vec<_>>()
        ))?;
        builder.build(config).await
    }

    fn parse_db_type(url: &str) -> Result<String> {
        if url.starts_with("mysql://") {
            Ok("mysql".to_string())
        } else if url.starts_with("postgres://") ||  url.starts_with("postgresql://") {
            Ok("postgres".to_string())
        } else {
            Err(anyhow!("Cannot parse database type from URL: {}", url))
        }
    }

    pub fn list_db_types() -> Vec<String> {
        let registry = REGISTRY.read().unwrap();
        registry.keys().cloned().collect()
    }
}