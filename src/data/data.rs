use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub some_val: i64
}

#[derive(Deserialize, Debug)]
pub struct JsonDataFromWebUi {
    pub topic: String,
    pub url: String,
    pub description: String
}