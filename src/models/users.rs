use axum::{Json, async_trait, extract::FromRequestParts, http::request::Parts, response::{IntoResponse, Response}};
use jsonwebtoken::{decode, DecodingKey, Validation};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

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

// Для JWT
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Кого ідентифікує токен
    pub role: String,
    pub exp: usize, // Час життя токена
}

pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Невірні дані"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Відсутні дані авторизації"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Помилка створення токена"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Недійсний або прострочений токен"),
        };
        let body = Json(serde_json::json!({
            "status": "error",
            "message": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingCredentials)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AuthError::InvalidToken);
        }

        let token = &auth_header[7..]; 

        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AuthError::TokenCreation)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}