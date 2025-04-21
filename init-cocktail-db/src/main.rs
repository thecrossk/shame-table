mod database;

use std::env;

use database::db;
use dotenv::dotenv;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let database_url = env::var("DATABASE_URL").expect("Variable 'DATABASE_URL' is not set");
    let tmp = "postgres://crosskdb:ming@192.168.2.104:5432/cocktailbar";
    let database = db::init_database_connection(&tmp).await?;
    tracing::info!("Connected to database on Pi, start running migrations...");

    sqlx::migrate!().run(&database).await.expect("Migrations failed");

    Ok(())
}
