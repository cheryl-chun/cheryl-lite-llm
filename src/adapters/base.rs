use std::pin::Pin;
use std::sync::Arc;

use futures_util::StreamExt;

use crate::adapters::{EventStream, LLMProvider};
use crate::adapters::builder::ProviderBuilder;
use crate::error::{ProxyError, Result};
use crate::models::{ChatRequest, ChatResponse};
use async_trait::async_trait;
use axum::body::{Body, Bytes};
use axum::response::sse::Event;
use reqwest::Client;

pub struct BaseProvider {
    client: Client,
    name: String,
    api_key: String,
    base_url: String,
}

pub struct BaseProviderBuilder {
    default_base_url: String,
}

impl BaseProviderBuilder {
    pub fn new(default_base_url: String) -> Self {
        Self { default_base_url }
    }
}

impl ProviderBuilder for BaseProviderBuilder {
    fn default_base_url(&self) -> &str {
        &self.default_base_url
    }

    fn build(
        &self,
        name: String,
        config: &crate::config::ProviderConfig,
    ) -> anyhow::Result<std::sync::Arc<dyn LLMProvider>> {
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| self.default_base_url.clone());
        let provider = BaseProvider::new(name, config.api_key.clone(), base_url);
        Ok(Arc::new(provider))
    }
}

impl BaseProvider {
    pub fn new(name: String, api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            name,
            api_key,
            base_url,
        }
    }
}

#[async_trait]
impl LLMProvider for BaseProvider {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ProxyError::Provider(format!(
                "{} API error ({}): {}",
                self.name, status, error_text
            )));
        }

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    }

    async fn chat_stream(&self, request: ChatRequest) -> Result<Body> {
        let mut request_body = request;
        request_body.stream = Some(true);

        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ProxyError::Provider(format!(
                "{} API error ({}): {}", self.name, status, error_text
            )));
        }

        let stream = response.bytes_stream();
        Ok(Body::from_stream(stream))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn supports_model(&self, _model: &str) -> bool {
        // BaseProvider 不做模型过滤，由路由层控制
        true
    }
}
