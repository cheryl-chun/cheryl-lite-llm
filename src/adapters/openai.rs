use crate::adapters::{BaseProviderBuilder, ProviderFactory};
use std::sync::Arc;

pub fn register() {
    let builder = Arc::new(BaseProviderBuilder::new(
        "https://api.openai.com/v1".to_string(),
    ));
    ProviderFactory::register("openai", builder);
}
