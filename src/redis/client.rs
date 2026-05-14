use redis::{AsyncCommands, Client, FromRedisValue, aio::Connection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Redis connection error: {0}")]
    Connection(String),

    #[error("Redis operation error: {0}")]
    Operation(String),
}

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(url: &str) -> Result<Self, RedisError> {
        let client = Client::open(url).map_err(|e| RedisError::Connection(e.to_string()))?;
        Ok(Self { client })
    }

    /// 获取 Redis 连接
    async fn get_conn(&self) -> Result<Connection, RedisError> {
        self.client
            .get_async_connection()
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))
    }

    pub async fn eval_script<T>(
        &self,
        script: &str,
        keys: Vec<String>,
        args: Vec<String>,
    ) -> Result<T, RedisError>
    where
        T: FromRedisValue,
    {
        let mut conn = self.get_conn().await?;

        let script = redis::Script::new(script);
        let mut invocation = script.prepare_invoke();
        for key in keys {
            invocation.key(key);
        }

        for arg in args {
            invocation.arg(arg);
        }

        invocation
            .invoke_async(&mut conn)
            .await
            .map_err(|e| RedisError::Operation(e.to_string()))
    }

    /// GET 操作
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, RedisError>
    where 
        T: FromRedisValue,
    {
        let mut conn = self.get_conn().await?;
        conn.get(key).await
            .map_err(|e| RedisError::Operation(e.to_string()))
    }

    /// SET 操作
    pub async fn set<V>(&self, key: &str, value: V) -> Result<(), RedisError>
    where 
        V: redis::ToRedisArgs + Send + Sync,
    {
        let mut conn = self.get_conn().await?;
        conn.set(key, value).await
            .map_err(|e| RedisError::Operation(e.to_string()))
    }

    /// SET 带有超时时间
    pub async fn set_ex<V>(&self, key: &str, value: V, ttl: u64) -> Result<(), RedisError>
    where 
        V: redis::ToRedisArgs + Send + Sync,
    {
        let mut conn = self.get_conn().await?;
        conn.set_ex(key, value, ttl).await
            .map_err(|e| RedisError::Operation(e.to_string()))
    }
}
