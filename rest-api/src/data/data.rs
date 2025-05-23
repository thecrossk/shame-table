use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize)]
pub struct ShameTableData {
    pub id: i64,
    pub topic: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>
}

pub enum Unit {
    Gram,
    Liter,
    Centiliter,
    Milliliter,
}

pub enum IngredientType {
    Water,
    Cola,
    Sugar,
    Meth,
}

pub struct Ingredient {
    ingredient: IngredientType,
    unit: Unit,
}

pub struct Recipe {
    ingredients: Vec<Ingredient>
}