use std::{fs, sync::Arc};

use axum::{extract::State, response::{Html, IntoResponse}, Json};
use sqlx::pool;

use crate::data::data::{AppState, JsonDataFromWebUi, ShameTableData};

pub async fn entries(
    State(state): State<Arc<AppState>>) -> Html<String> {
    let html_content = fs::read_to_string("src/html/results.html").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());

    Html(html_content)
}