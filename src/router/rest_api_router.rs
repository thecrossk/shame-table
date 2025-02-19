use std::sync::Arc;

use axum::{Router, routing::{post, get}};

use tower_http::services::ServeDir;

use crate::{data::data::AppState, route_handler::{cocktails::cocktail_page, entries::entries, get_entries::get_entries}};
use crate::route_handler::add_entry::add_entry;
use crate::route_handler::start_page::get_start_page;

pub fn create_router(state: Arc<AppState>) -> anyhow::Result<Router> {
    let router = Router::new()

        .nest_service("/images", ServeDir::new("src/assets/images/cocktailbar"))

        .route("/", get(get_start_page))
        .route("/add-entry", post(add_entry))
        .route("/get-entries", get(get_entries))
        .route("/entries", get(entries))
        .route("/cocktails", get(cocktail_page))

        .with_state(state);

    Ok(router)
}