use crate::database::{VirtualAuthContext, MasterAuthContext};
use crate::error::{Result, ProxyError};
use crate::models::{ChatRequest, ChatResponse};
use crate::server::state::AppState;
use axum::{
    extract::{Json, Path, State, Extension},
    http::StatusCode,
};
use uuid::Uuid;

// ============= LLM API Handlers（需要 Virtual Key）=============

/// 聊天接口
pub async fn chat_handler(
    Path(provider): Path<String>,
    State(state): State<AppState>,
    Extension(auth_ctx): Extension<VirtualAuthContext>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>> {
    // 检查用户是否有权限使用这个模型
    if !auth_ctx.models.is_empty()
        && !auth_ctx.models.contains(&"*".to_string())
        && !auth_ctx.models.contains(&request.model)
    {
        return Err(ProxyError::Auth(
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

// ============= Admin API Handlers（需要 Master Key）=============

/// 创建 Virtual Key
pub async fn create_virtual_key_handler(
    State(_state): State<AppState>,
    Extension(_auth_ctx): Extension<MasterAuthContext>,
    Json(_request): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    // TODO: 实现创建逻辑
    Ok((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Not implemented yet"
        }))
    ))
}

/// 列出所有 Virtual Keys
pub async fn list_virtual_keys_handler(
    State(_state): State<AppState>,
    Extension(_auth_ctx): Extension<MasterAuthContext>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现列表查询
    Ok(Json(serde_json::json!({
        "keys": []
    })))
}

/// 获取 Virtual Key 详情
pub async fn get_virtual_key_handler(
    State(_state): State<AppState>,
    Extension(_auth_ctx): Extension<MasterAuthContext>,
    Path(_key_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // TODO: 实现详情查询
    Ok(Json(serde_json::json!({
        "error": "Not implemented yet"
    })))
}

/// 撤销 Virtual Key
pub async fn revoke_virtual_key_handler(
    State(_state): State<AppState>,
    Extension(_auth_ctx): Extension<MasterAuthContext>,
    Path(_key_id): Path<Uuid>,
) -> Result<StatusCode> {
    // TODO: 实现撤销逻辑
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// 删除 Virtual Key
pub async fn delete_virtual_key_handler(
    State(_state): State<AppState>,
    Extension(_auth_ctx): Extension<MasterAuthContext>,
    Path(_key_id): Path<Uuid>,
) -> Result<StatusCode> {
    // TODO: 实现删除逻辑
    Ok(StatusCode::NOT_IMPLEMENTED)
}

// ============= Public Handlers（不需要认证）=============

/// 健康检查
pub async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
