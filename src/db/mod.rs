use sqlx::postgres::PgPoolOptions;
use std::env;

pub type DbPool = sqlx::PgPool;

pub mod models;
pub mod queries;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
