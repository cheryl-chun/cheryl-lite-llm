use crate::database::AuthContext;
use crate::error::Result;
use crate::models::{ChatRequest, ChatResponse};
use crate::server::state::AppState;
use axum::{
    extract::{Json, Path, State, Extension},
    http::StatusCode,
};

pub async fn chat_handler(
    Path(provider): Path<String>,
    State(state): State<AppState>,
    Extension(auth_ctx): Extension<AuthContext>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>> {
    // 检查用户是否有权限使用这个模型
    if !auth_ctx.models.is_empty() && !auth_ctx.models.contains(&request.model) {
        return Err(crate::error::ProxyError::Auth(
            format!("Model '{}' not allowed for this API key", request.model)
        ));
    }

    let model = request.model.clone();

    // 调用 router 处理请求
    let response = state.router.chat(&provider, request).await?;

    tracing::info!(
        "User {:?} called {}/{} - tokens: {:?}",
        auth_ctx.user_id,
        provider,
        model,
        response.usage
    );

    Ok(Json(response))
}

// 健康检查
pub async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
