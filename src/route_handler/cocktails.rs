use std::fs;

use axum::response::Html;

pub async fn cocktail_page() -> Html<String> {
    let html_content = fs::read_to_string("src/html/cocktail_machine/cocktails.html").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());
    Html(html_content)
}