use serde::Deserialize;
use sqlx::{PgPool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Deserialize, Debug)]
pub struct JsonDataFromWebUi {
    pub topic: String,
    pub url: String,
    pub description: String
}