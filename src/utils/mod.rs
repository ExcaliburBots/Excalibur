use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;

pub async fn obtain_pool(url: &str) -> Result<PgPool, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(10) // maximum number of connections in the pool
        .connect(url)
        .await?;

    Ok(pool)
}
