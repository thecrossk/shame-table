use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use anyhow::Result;

pub async fn init_database_connection(url: &str) -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    Ok(pool)
}