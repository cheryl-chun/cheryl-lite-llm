mod handler;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub use state::AppState;

use crate::middleware;


pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/:provider/chat/completions", post(handler::chat_handler))
        .route("/health", get(handler::health_handler))
        .layer(axum::middleware::from_fn_with_state(app_state.clone(), middleware::auth_middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
