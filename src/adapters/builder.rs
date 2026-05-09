use crate::adapters::LLMProvider;
use crate::config::ProviderConfig;
use anyhow::Result;
use std::sync::Arc;

pub trait ProviderBuilder: Send + Sync {
    fn default_base_url(&self) -> &str;
    fn build(&self, name: String, config: &ProviderConfig) -> Result<Arc<dyn LLMProvider>>;
}
