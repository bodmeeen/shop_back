use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Product {
    pub id: i64, // В майбутньому можна буде замінити на UUID
    pub title: String,
    pub body: String,
    pub old_price: Option<i64>, // Option, бо може бути NULL
    pub price: i64,
    pub status: String,
    // потрібно буде дізнатись як правильно тут зробити created_at
    // pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}