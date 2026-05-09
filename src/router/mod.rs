use crate::adapters::LLMProvider;
use crate::error::{ProxyError, Result};
use crate::models::{ChatRequest, ChatResponse};
use std::collections::HashMap;
use std::sync::Arc;

// 路由器：管理所有 provider
pub struct Router {
    providers: HashMap<String, Arc<dyn LLMProvider>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    // 注册一个 provider
    pub fn register(&mut self, provider: Arc<dyn LLMProvider>) {
        let name = provider.name().to_string();
        self.providers.insert(name, provider);
    }

    // 根据供应商返回 provider
    fn find_provider(&self, provider_name: &str) -> Result<&Arc<dyn LLMProvider>> {
        self.providers
            .get(provider_name)
            .ok_or_else(|| ProxyError::InvalidModel(provider_name.to_string()))
    }

    pub async fn chat(&self, provider_name: &str, request: ChatRequest) -> Result<ChatResponse> {
        let provider = self.find_provider(provider_name)?;
        provider.chat(request).await
    }
}
