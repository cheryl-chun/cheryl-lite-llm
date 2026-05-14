use std::{convert::Infallible, pin::Pin};

use futures_util::{Stream, StreamExt};
use crate::{error, models::{ChatRequest, ChatResponse}};
use async_trait::async_trait;
use axum::{body::{Body, Bytes}, response::sse::Event};

pub type EventStream =
    Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>;

// LLM 供应商统一接口
#[async_trait]
pub trait LLMProvider: Send + Sync {
    // 普通聊天请求
    async fn chat(&self, request: ChatRequest) -> error::Result<ChatResponse>;

    /// 流式请求（直接透传 upstream stream）
    async fn chat_stream(&self, request: ChatRequest) -> error::Result<Body>;

    // 获取供应商名称
    fn name(&self) -> &str;

    // 模型是否支持
    fn supports_model(&self, model: &str) -> bool;
}
