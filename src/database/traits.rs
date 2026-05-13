use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    database::models::{MasterKey, VirtualKey, VirtualAuthContext},
    error::Result
};

/// Virtual Key Repository trait
#[async_trait]
pub trait VirtualKeyRepository: Send + Sync {
    /// 根据 key hash 查找并验证 Virtual Key
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<VirtualAuthContext>>;

    /// 列出所有 Virtual Keys
    async fn list_all(&self) -> Result<Vec<VirtualKey>>;

    /// 创建新的 Virtual Key
    async fn create(&self, key: &VirtualKey) -> Result<()>;

    /// 禁用 Virtual Key
    async fn disable(&self, key_id: &Uuid) -> Result<()>;

    /// 启用 Virtual Key
    async fn enable(&self, key_id: &Uuid) -> Result<()>;

    /// 删除 Virtual Key
    async fn delete(&self, key_id: &Uuid) -> Result<()>;
}

/// Master Key Repository trait
#[async_trait]
pub trait MasterKeyRepository: Send + Sync {
    /// 根据 key hash 查找 Master Key
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<MasterKey>>;

    /// 列出所有 Master Keys
    async fn list_all(&self) -> Result<Vec<MasterKey>>;

    /// 创建新的 Master Key
    async fn create(&self, key: &MasterKey) -> Result<()>;

    /// 禁用 Master Key
    async fn disable(&self, key_id: &Uuid) -> Result<()>;

    /// 启用 Master Key
    async fn enable(&self, key_id: &Uuid) -> Result<()>;
}