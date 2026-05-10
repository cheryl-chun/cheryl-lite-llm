use crate::{error::ProxyError, server::AppState};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use reqwest::header::AUTHORIZATION;

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, ProxyError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ProxyError::Auth("Missing Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ProxyError::Auth("Invalid token format".to_string()));
    }

    if valid_token(auth_header) {
        let response = next.run(req).await;
        Ok(response)
    } else {
        Err(ProxyError::Auth(format!(
            "Invalid or expired API key: {}",
            auth_header
        )))
    }
}

// TODO: Token 校验
fn valid_token(token: &str) -> bool {
    let token = token.trim_start_matches("Bearer ").trim();
    return false;
}
