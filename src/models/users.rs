use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Users {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email: String,
    pub password_hash: String,
    pub phone_number: i64,
    pub created_at: Option<String>
}