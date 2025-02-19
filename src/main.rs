
/*
#[derive(Debug, FromRow)]
pub struct ShameTableData {
    pub id: i64,
    pub topic: String,
    pub url: String,
    pub description: String
}


*/

mod database;
mod router;
mod data;
mod route_handler;

use std::net::SocketAddr;
use std::sync::Arc;

use data::data::AppState;
use database::db;


use router::rest_api_router::create_router;
use tracing::{debug, error, info};
use tracing_subscriber::fmt;
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let rest_api_router = create_router(app_state)?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, rest_api_router).await?;
    /*
        

    
    println!("Server running at http://{}", addr);


    


    let res = sqlx::query("SELECT * FROM shame_table").fetch_all(&pool).await?;
    println!("Rows fetches: {}", res.len());

    for row in res {
        println!("row {:?}",row);
    }

    
    */
    Ok(())
}
