use crate::database::{MasterAuthContext, VirtualAuthContext, VirtualKey};
use crate::error::{ProxyError, Result};
use crate::models::{ChatRequest, ChatResponse};
use crate::server::state::AppState;
use crate::server::{
    CreateVirtualKeyRequest, CreateVirtualKeyResponse, GetKeysByUserResponse, GetVirtualKeyResponse, ListVirtualKeyResponse, VirtualKeyInfo
};
use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
};
use chrono::{Duration, Utc};
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
        return Err(ProxyError::Auth(format!(
            "Model '{}' not allowed for this API key",
            request.model
        )));
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
    State(state): State<AppState>,
    Extension(master_ctx): Extension<MasterAuthContext>,
    Json(request): Json<CreateVirtualKeyRequest>,
) -> Result<(StatusCode, Json<CreateVirtualKeyResponse>)> {
    use crate::utils::{compute_sha256, generate_random_key};

    let raw_key = generate_random_key("sk-");
    let key_hash = compute_sha256(&raw_key);

    let expires_at = if request.expires_in_days > 0 {
        Some(Utc::now() + Duration::days(request.expires_in_days as i64))
    } else {
        None
    };

    let virtual_key = VirtualKey {
        id: Uuid::new_v4(),
        key_hash,
        enabled: true,
        expires_at,
        models: request.models,
        user_id: request.user_id,
        team_id: request.team_id,
        created_by: master_ctx.key_id,
        description: request.description,
        created_at: Utc::now(),
        last_used_at: None,
    };

    state
        .db
        .virtual_key_repo
        .create(&virtual_key)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create virtual key: {}", e);
            ProxyError::Database(format!("Failed to create virtual key: {}", e))
        })?;

    tracing::info!(
        "Virtual key created: id={}, user_id={:?}, created_by={}",
        virtual_key.id,
        virtual_key.user_id,
        master_ctx.key_id
    );

    let response = CreateVirtualKeyResponse {
        key: raw_key,
        key_id: virtual_key.id,
        created_at: virtual_key.created_at.to_rfc3339(),
        expires_at: virtual_key.expires_at.map(|t| t.to_rfc3339()),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// 列出所有 Virtual Keys
pub async fn list_virtual_keys_handler(
    State(state): State<AppState>,
    Extension(master_ctx): Extension<MasterAuthContext>,
) -> Result<Json<ListVirtualKeyResponse>> {
    let vks = state.db.virtual_key_repo.list_all().await.map_err(|e| {
        tracing::error!("Failed to list all virtual keys");
        ProxyError::Database(format!("Failed to list virtual keys: {}", e))
    })?;

    let vk_infos: Vec<VirtualKeyInfo> = vks
        .into_iter()
        .map(|key| VirtualKeyInfo {
            id: key.id,
            user_id: key.user_id,
            team_id: key.team_id,
            models: key.models,
            enabled: key.enabled,
            created_at: key.created_at.to_rfc3339(),
            expires_at: key.expires_at.map(|t| t.to_rfc3339()),
            last_used_at: key.last_used_at.map(|t| t.to_rfc3339()),
            description: key.description,
            created_by: key.created_by,
        })
        .collect();

    let total = vk_infos.len();

    tracing::info!("Listed {} virtual keys", total);
    Ok(Json(ListVirtualKeyResponse {
        keys: vk_infos,
        total,
    }))
}

/// 获取 Virtual Key 详情
pub async fn get_virtual_key_handler(
    State(state): State<AppState>,
    Extension(auth_ctx): Extension<MasterAuthContext>,
    Path(key_id): Path<Uuid>,
) -> Result<Json<GetVirtualKeyResponse>> {
    let key = state
        .db
        .virtual_key_repo
        .find_by_id(&key_id)
        .await?
        .ok_or_else(|| ProxyError::NotFound(format!("Virtual key not found: {}", key_id)))?;

    Ok(Json(GetVirtualKeyResponse {
        key: VirtualKeyInfo {
            id: key.id,
            user_id: key.user_id,
            team_id: key.team_id,
            models: key.models,
            enabled: key.enabled,
            created_at: key.created_at.to_rfc3339(),
            expires_at: key.expires_at.map(|t| t.to_rfc3339()),
            last_used_at: key.last_used_at.map(|t| t.to_rfc3339()),
            description: key.description,
            created_by: key.created_by,
        },
    }))
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

/// 根据 User ID 获取 Keys（改进版）
pub async fn get_keys_by_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<GetKeysByUserResponse>> {
    // 直接根据 user_id 查询
    let keys = state.db.virtual_key_repo.find_by_user_id(&user_id).await?;

    let key_infos: Vec<VirtualKeyInfo> = keys
        .into_iter()
        .map(|key| VirtualKeyInfo {
            id: key.id,
            user_id: key.user_id,
            team_id: key.team_id,
            models: key.models,
            enabled: key.enabled,
            created_at: key.created_at.to_rfc3339(),
            expires_at: key.expires_at.map(|t| t.to_rfc3339()),
            last_used_at: key.last_used_at.map(|t| t.to_rfc3339()),
            description: key.description,
            created_by: key.created_by,
        })
        .collect();

    let total = key_infos.len();

    Ok(Json(GetKeysByUserResponse {
        user_id,
        keys: key_infos,
        total,
    }))
}

// ============= Public Handlers（不需要认证）=============

/// 健康检查
pub async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}