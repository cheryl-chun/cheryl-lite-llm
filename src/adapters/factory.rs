use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::adapters::{LLMProvider, builder::ProviderBuilder};
use crate::config::ProviderConfig;
use anyhow::{Result, anyhow};

static REGISTRY: Lazy<RwLock<HashMap<String, Arc<dyn ProviderBuilder>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn register(name: &str, builder: Arc<dyn ProviderBuilder>) {
        let mut registry = REGISTRY.write().unwrap();
        registry.insert(name.to_string(), builder);
    }

    // 根据名称创建 provider
    pub fn create(provider_type: &str, config: &ProviderConfig) -> Result<Arc<dyn LLMProvider>> {
        let registry = REGISTRY.read().unwrap();

        let builder = registry.get(provider_type).ok_or_else(|| {
            anyhow!(
                "Unknown provider type: {}. Available: {:?}",
                provider_type,
                registry.keys().collect::<Vec<_>>()
            )
        })?;

        builder.build(provider_type.to_string(), config)
    }

    // 列出所有已注册的 provider
    pub fn list_providers() -> Vec<String> {
        let registry = REGISTRY.read().unwrap();
        registry.keys().cloned().collect()
    }
}
