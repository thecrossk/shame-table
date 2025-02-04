use axum::{
    response::{Html, IntoResponse, Response}, routing::{get, post}, Json, Router
};
use sqlx::{
    FromRow,
    postgres::PgPoolOptions,
    query,
    query_as
};

use std::{
    fs, net::SocketAddr,
};

use serde_json::{
    Deserializer,
    Serializer,
    Value
};

use serde::Deserialize;

pub struct ShameTableData {
    pub id: i64,
    pub topic: String,
    pub url: String,
    pub description: String
}

#[derive(Deserialize)]
pub struct JsonDataFromWebUi {
    pub topic: String,
    pub url: String,
    pub description: String
}

async fn get_start_page() -> Html<String> {
    let html_content = fs::read_to_string("src/html/index.html").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());
    Html(html_content)
}

async fn add_entry(Json(payload): Json<JsonDataFromWebUi>) -> impl IntoResponse {
    println!("web ui handler hello there");
    "some literal into response"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting my little rest-api server...");
    let app = Router::new()
        .route("/", get(get_start_page))
        .route("/add-entry", post(add_entry));
        

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let database_url = "postgres://crosskdb:ming@192.168.178.56:5432/crossk-db";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Connected to database on Pi");

    let rows = sqlx::query(
        r#"
            SELECT * FROM shame_table
        "#
    )
    .execute(&pool)
    .await?;

    println!("Fetched {} rows from table shame_table", rows.rows_affected());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
