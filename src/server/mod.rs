mod handler;
mod state;
mod request;

use axum::{
    Router,
    routing::{get, post, delete},
    middleware,
};
use tower_http::trace::TraceLayer;

pub use state::AppState;
pub use request::*;

use crate::middleware::{master_auth_middleware, virtual_auth_middleware};

/// 创建应用路由
///
/// 路由分为三组：
/// 1. 管理路由（/admin/*）- 需要 Master Key 认证
/// 2. LLM 路由（/:provider/*）- 需要 Virtual Key 认证
/// 3. 公开路由（/health）- 不需要认证
pub fn create_router(app_state: AppState) -> Router {
    // 1. 管理路由组（需要 Master Key）
    let admin_routes = Router::new()
        .route("/admin/keys", post(handler::create_virtual_key_handler))
        .route("/admin/keys", get(handler::list_virtual_keys_handler))
        .route("/admin/keys/:key_id", get(handler::get_virtual_key_handler))
        .route("/admin/keys/:key_id/revoke", post(handler::revoke_virtual_key_handler))
        .route("/admin/keys/:key_id", delete(handler::delete_virtual_key_handler))
        .route("/admin/keys/by-user/:user_id", get(handler::get_keys_by_user_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            master_auth_middleware,
        ));

    // 2. LLM 路由组（需要 Virtual Key）
    let llm_routes = Router::new()
        .route("/:provider/chat/completions", post(handler::chat_handler))
        .route("/:provider/chat/completions/stream", post(handler::chat_stream_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            virtual_auth_middleware,
        ));

    // 3. 公开路由（不需要认证）
    let public_routes = Router::new()
        .route("/health", get(handler::health_handler));

    // 4. 合并所有路由
    Router::new()
        .merge(admin_routes)
        .merge(llm_routes)
        .merge(public_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
