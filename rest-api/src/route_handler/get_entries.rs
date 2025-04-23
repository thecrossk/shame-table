use std::{fs, sync::Arc};

use axum::{extract::State, response::{Html, IntoResponse}, Json};
use sqlx::pool;

use crate::data::data::{AppState, JsonDataFromWebUi, ShameTableData};

pub async fn get_entries(
    State(state): State<Arc<AppState>>) -> Json<Vec<ShameTableData>> {
    /*
    let rows = sqlx::query_as!(
        ShameTableData,
        r#"SELECT * FROM shame_table"#
    ).fetch_all(&state.pool)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(rows)
    */
    Json(vec![])
}
