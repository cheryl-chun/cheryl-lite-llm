use async_trait::async_trait;

mod redis_limiter;
mod lua_scriptes;
mod types;

pub use types::{RateLimitError, LimitResult};

use crate::redis::RedisClient;

#[async_trait]
pub trait LimitStrategy: Send + Sync {
    /// 检查并记录
    async fn check(&self, client: &RedisClient, key_id: &str) -> Result<LimitResult, RateLimitError>;
}