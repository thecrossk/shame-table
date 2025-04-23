use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::data::data::{AppState, JsonDataFromWebUi};

pub async fn add_entry(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JsonDataFromWebUi>) -> impl IntoResponse {
    tracing::info!("Received json payload: {:?}", payload);

    let topic = payload.topic;
    let url = payload.url;
    let desc = payload.description;
/*
    let affected_rows = sqlx::query!(
        r#"INSERT INTO shame_table (topic, url, description) VALUES ($1, $2, $3)"#,
        topic,
        url,
        desc
    ).execute(&state.pool).await;
*/
    "some literal into response"
}