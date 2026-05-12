pub mod adapters;
pub mod config;
pub mod error;
pub mod middleware;
pub mod models;
pub mod router;
pub mod server;
pub mod utils;
pub mod database;
pub mod keys;

pub fn init() {
    adapters::init_providers();
    database::init_repositories();
}