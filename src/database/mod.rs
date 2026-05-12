mod traits;
mod models;
mod pg_repo;
mod mysql_repo;
mod builder;
mod context;
mod factory;

pub use models::{AuthContext, VirtualKeyRow};
pub use traits::AuthRepository;
pub use pg_repo::PgRepository;
pub use mysql_repo::MySqlRepository;
pub use context::{DatabaseContext, DatabasePool};
pub use factory::{DatabaseFactory};

pub fn init_repositories() {
    pg_repo::register();
    mysql_repo::register();
}