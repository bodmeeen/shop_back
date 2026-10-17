use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub created_at: Option<String>
}

#[derive(Deserialize)]
pub struct CreateUserSchema {
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: String,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,
}



// Схема для отримання даних при реєстрації користувача,
// а особливо нехешований пароль
#[derive(Deserialize)]
pub struct RegisterUserSchema {
    pub first_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}