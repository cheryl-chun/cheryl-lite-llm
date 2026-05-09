use crate::error::Result;
use crate::models::{ChatRequest, ChatResponse};
use async_trait::async_trait;

// LLM 供应商统一接口
#[async_trait]
pub trait LLMProvider: Send + Sync {
    // 普通聊天请求
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    // 获取供应商名称
    fn name(&self) -> &str;

    // 模型是否支持
    fn supports_model(&self, model: &str) -> bool;
}
