use std::sync::Arc;

use axum::{Router, routing::{post, get}};

use crate::data::data::AppState;
use crate::route_handler::add_entry::add_entry;
use crate::route_handler::start_page::get_start_page;

pub fn create_router(state: Arc<AppState>) -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/", get(get_start_page))
        .route("/add-entry", post(add_entry))
        .with_state(state);

    Ok(router)
}