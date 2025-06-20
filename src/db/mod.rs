pub mod api;
mod models;
pub use models::*;
pub mod schema;

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn create_pool(database_url: &str) -> DbPool {
    let manager = AsyncDieselConnectionManager::new(database_url);
    Pool::builder(manager)
        .max_size(4)
        .build()
        .expect("Failed to create pool")
}
