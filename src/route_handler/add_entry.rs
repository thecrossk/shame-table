use axum::{response::IntoResponse, Json};

use crate::data::data::JsonDataFromWebUi;

pub async fn add_entry(Json(payload): Json<JsonDataFromWebUi>) -> impl IntoResponse {
    println!("web ui handler hello there");
    "some literal into response"
}