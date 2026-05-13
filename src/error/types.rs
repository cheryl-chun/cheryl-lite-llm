use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("Provider error: {0}")]
    Provider(String),

    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid model: {0}")]
    InvalidModel(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Database error: {0}")]
    Database(String),
}

impl axum::response::IntoResponse for ProxyError {
    fn into_response(self) -> axum::response::Response {
        use axum::Json;
        use axum::http::StatusCode;
        use serde_json::json;

        let (status, error_message) = match self {
            ProxyError::Auth(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            ProxyError::InvalidModel(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ProxyError::Provider(_) | ProxyError::HttpRequest(_) => {
                (StatusCode::BAD_GATEWAY, self.to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": "proxy_error",
            }
        }));

        (status, body).into_response()
    }
}
