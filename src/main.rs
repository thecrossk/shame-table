use axum::{
    response::Html,
    routing::get,
    Router,
};

use std::{
    fs, net::SocketAddr,
};

use serde::{Deserialize, Serialize};

use serde_json::Value;
use sqlx::FromRow;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, FromRow)]
pub struct TableData {
    pub id: i64,
    pub data: Value,
    pub description: String
}

#[derive(Deserialize)]
pub struct ShameTableData {
    pub id: i64,
    pub topic: String,
    pub url: String,
    pub description: String
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String
}

async fn handle_add_entry(Json(payload): Json<ShameTableData>) -> impl IntoResponse {
    let response = format!(
            "Received params: {}, {}, {}",
            payload.topic, payload.url, payload.description
        );

    Json(ResponseMessage {
        message: response,
    })
}

async fn get_start_page() -> Html<String> {
    let html_content = fs::read_to_string("src/html/index.html").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());
    Html(html_content)
}

#[tokio::main]
async fn main() {
    println!("Starting my little rest-api server...");
    let app = Router::new()
        .route("/", get(get_start_page))
        .route("/add-entry", post(handle_add_entry));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let database_url = "postgres://crosskdb:ming@localhost:5432/crossk-db";

    let database = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
