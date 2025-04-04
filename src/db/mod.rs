use sqlx::postgres::PgPoolOptions;
use std::env;

pub type DbPool = sqlx::PgPool;

pub async fn init_db -> Result<DbPool, sqlx::Error> {
  
}