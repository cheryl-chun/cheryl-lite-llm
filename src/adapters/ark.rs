use crate::adapters::{BaseProviderBuilder, ProviderFactory};
use std::sync::Arc;

// Ark provider 的注册
pub fn register() {
    let builder = Arc::new(BaseProviderBuilder::new(
        "https://ark.cn-beijing.volces.com/api/v3".to_string(),
    ));
    ProviderFactory::register("ark", builder);
}
