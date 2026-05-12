use crate::{database::AuthContext, error::ProxyError, server::AppState};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use reqwest::header::AUTHORIZATION;
use sha2::{Digest, Sha256};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ProxyError> {
    // 提取 Authorization Header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ProxyError::Auth("Missing Authorization header".to_string()))?;

    // 检查格式
    if !auth_header.starts_with("Bearer ") {
        return Err(ProxyError::Auth("Invalid token format".to_string()));
    }

    // 提取 token去除掉 "Bearer " 和去除之后的前后空格
    let token = auth_header.trim_start_matches("Bearer ").trim();

    if token.is_empty() {
        return Err(ProxyError::Auth("Empty token".to_string()));
    }

    // 验证 token
    let auth_context = validate_token(&state, token).await?;

    req.extensions_mut().insert(auth_context);

    let response = next.run(req).await;
    Ok(response)
}

async fn validate_token(state: &AppState, token: &str) -> Result<AuthContext, ProxyError> {
    let key_hash = compute_key_hash(token);
    let auth_context_opt = state.db.auth_repo.validate_key(&key_hash)
    .await
    .map_err(|e| ProxyError::Auth(format!("Failed to validate token: {}", e)))?;

    let auth_context = auth_context_opt.ok_or_else(|| ProxyError::Auth("Invalid API key".to_string()))?;

    // 检查是否可用
    if !auth_context.enabled {
        return Err(ProxyError::Auth("API key is disabled".to_string()));
    }

    // 检查是否过期
    if let Some(expires_at) = auth_context.expires_at {
        let now = chrono::Utc::now();
        if now > expires_at {
            return Err(ProxyError::Auth("API key has expired".to_string()));
        }
    }

    // TODO: 其他检查

    Ok(auth_context)
}

fn compute_key_hash(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}