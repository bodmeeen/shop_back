use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Product {
    pub id: i64, // В майбутньому можна буде замінити на UUID
    pub title: String,
    pub body: String,
    pub old_price: Option<i64>, // Option, бо може бути NULL
    pub price: i64,
    pub status: String,
    // потрібно буде дізнатись як правильно тут зробити created_at
    pub created_at: Option<String>,
    // Це можна буде повернути коли потрібно буде в беку
    // робити якісь дії з датою, порівняння дат і тд.
    // pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct CreateProductSchema {
    pub title: String,
    pub body: String,
    pub old_price: Option<i64>, // Option, бо може бути NULL
    pub price: i64,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductSchema {
    pub title: Option<String>,
    pub body: Option<String>,
    pub price: Option<i64>,
    pub status: Option<String>,
    // потрібно буде додати created_at
}