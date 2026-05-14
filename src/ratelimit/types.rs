use thiserror::Error;

#[derive(Debug, Clone)]
pub struct LimitResult {
    pub allowed: bool,
    pub current: u64,
    pub limit: u64,
    pub reset_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct QuotaInfo {
    pub used: u64,
    pub limit: u64,
    pub reset_at: i64,
}

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Redis connection error: {0}")]
    Connection(String),

    #[error("Redis operation error: {0}")]
    Operation(String),

    #[error("Script execution error: {0}")]
    Script(String),
}

impl LimitResult {
    pub fn remaining(&self) -> u64 {
        if self.current >= self.limit {
            0
        } else {
            self.limit - self.current
        }
    }
}
