mod database;
mod router;
mod data;
mod route_handler;

use std::net::SocketAddr;
use std::sync::Arc;

use data::data::AppState;
use database::db;


use router::rest_api_router::create_router;
use tracing_subscriber::EnvFilter;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    println!("Starting my little rest-api server...");

    let database_url = "postgres://crosskdb:ming@192.168.178.56:5432/crossk-db";
    let database = db::init_database_connection(&database_url).await?;
    tracing::info!("Connected to database on Pi");

    let app_state = Arc::new(AppState {pool: database});

    let _addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let rest_api_router = create_router(app_state)?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, rest_api_router).await?;

    Ok(())
}
