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
};

/// Virtual Key 认证中间件
/// 用于保护 LLM API（/:provider/chat/completions）
pub async fn virtual_auth_middleware(
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

    // 4. 验证是否是 Virtual Key 格式
    if !token.starts_with("sk-") {
        return Err(ProxyError::Auth(
            "Invalid Virtual Key format. Virtual Keys must start with 'sk_live_' or 'sk_test_'".to_string()
        ));
    }

    let key_hash = compute_key_hash(token);

    let virtual_key = state.db.virtual_key_repo.find_by_hash(&key_hash)
        .await
        .map_err(|e| ProxyError::Auth(format!("Failed to validate Virtual Key: {}", e)))?
        .ok_or_else(|| ProxyError::Auth("Invalid Virtual Key".to_string()))?;

    // 检查是否启用
    if !virtual_key.enabled {
        return Err(ProxyError::Auth("Virtual Key is disabled".to_string()));
    }

    // 检查是否过期
    if let Some(expires_at) = virtual_key.expires_at {
        let now = Utc::now();
        if now > expires_at {
            return Err(ProxyError::Auth("Virtual Key has expired".to_string()));
        }
    }

    tracing::debug!(
        "Virtual Key authenticated: key_id={}, user_id={:?}",
        virtual_key.key_id,
        virtual_key.user_id
    );

    req.extensions_mut().insert(virtual_key);

    let response = next.run(req).await;
    Ok(response)
}

fn compute_key_hash(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
