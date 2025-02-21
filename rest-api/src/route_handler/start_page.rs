use std::fs;

use axum::response::Html;

pub async fn get_start_page() -> Html<String> {
    let html_content = fs::read_to_string("rest-api/src/html/index.html").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());
    Html(html_content)
}