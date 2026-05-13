use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::header::AUTHORIZATION,
};
use sha2::{Sha256, Digest};
use chrono::Utc;

use crate::{
    error::ProxyError,
    server::AppState,
    database::MasterAuthContext,
};

/// Master Key 认证中间件
/// 用于保护管理 API（/admin/*）
pub async fn master_auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ProxyError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ProxyError::Auth("Missing Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ProxyError::Auth(
            "Invalid token format. Expected 'Bearer <token>'".to_string()
        ));
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();

    if token.is_empty() {
        return Err(ProxyError::Auth("Empty token".to_string()));
    }

    if !token.starts_with("mk_") {
        return Err(ProxyError::Auth(
            "Invalid Master Key format. Master Keys must start with 'mk_'".to_string()
        ));
    }

    let key_hash = compute_key_hash(token);

    let master_key = state.db.master_key_repo.find_by_hash(&key_hash)
        .await
        .map_err(|e| ProxyError::Auth(format!("Failed to validate Master Key: {}", e)))?
        .ok_or_else(|| ProxyError::Auth("Invalid Master Key".to_string()))?;

    // 检查是否启用
    if !master_key.enabled {
        return Err(ProxyError::Auth("Master Key is disabled".to_string()));
    }

    // 检查是否过期
    if let Some(expires_at) = master_key.expires_at {
        let now = Utc::now();
        if now > expires_at {
            return Err(ProxyError::Auth("Master Key has expired".to_string()));
        }
    }

    // 创建认证上下文
    let auth_context = MasterAuthContext {
        key_id: master_key.id,
        key_hash: master_key.key_hash,
        enabled: master_key.enabled,
        expires_at: master_key.expires_at,
        description: master_key.description,
    };

    req.extensions_mut().insert(auth_context);

    tracing::debug!("Master Key authenticated: key_id={}", master_key.id);

    let response = next.run(req).await;
    Ok(response)
}

fn compute_key_hash(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
