use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::error::Error;

pub async fn obtain_pool() -> Result<PgPool, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(10) // maximum number of connections in the pool
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    Ok(pool)
}
