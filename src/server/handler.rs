use crate::error::Result;
use crate::models::{ChatRequest, ChatResponse};
use crate::server::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

pub async fn chat_handler(
    Path(provider): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>> {
    // 调用 router 处理请求
    let response = state.router.chat(&provider, request).await?;

    Ok(Json(response))
}

// 健康检查
pub async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
