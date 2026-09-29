use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email: String,
    pub password_hash: String,
    pub phone_number: String,
    pub created_at: Option<String>
}

#[derive(Deserialize)]
pub struct CreateUserSchema {
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email: String,
    pub password_hash: String,
    pub phone_number: String,
}

// #[derive(Serialize, Deserialize, Debug)]