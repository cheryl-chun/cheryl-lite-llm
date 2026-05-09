pub mod types;

pub use types::ProxyError;

pub type Result<T> = std::result::Result<T, ProxyError>;
