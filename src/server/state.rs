use crate::router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub router: Arc<Router>,
}

impl AppState {
    pub fn new(router: Router) -> Self {
        Self {
            router: Arc::new(router),
        }
    }
}
