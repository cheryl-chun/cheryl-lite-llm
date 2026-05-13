mod master_auth;
mod virtual_auth;

pub use master_auth::master_auth_middleware;
pub use virtual_auth::virtual_auth_middleware;