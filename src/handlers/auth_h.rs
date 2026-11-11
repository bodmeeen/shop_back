use axum::{Json, extract::{State}, http::{StatusCode}, response::IntoResponse};
use crate::{AppState, models::users::{LoginUserSchema, RegisterUserSchema, User, Claims}};
use serde_json::json;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher
};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Duration, Utc};


pub async fn registration_user(State(state): State<AppState>,
        Json(payload): Json<RegisterUserSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {


        // Створення хешованого пароля при реєстрації
    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(payload.password.as_bytes())
        .expect("Помилка хешування пароля")
        .to_string();

    let result = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (first_name, email, password_hash, role) 
        VALUES ($1, $2, $3, $4)
        RETURNING *"#,
        )
        .bind(&payload.first_name)
        .bind(&payload.email)
        .bind(&hashed_password)
        .bind("customer")
        .fetch_one(&state.db)
        .await;

 match result {
        Ok(user) => {
            let role = user.role.as_deref().unwrap_or("customer");

            let token = match generate_jwt_token(user.id, role) {
                Ok(t) => t,
                Err(err) => {
                    let error_response = json!({
                        "status": "error",
                        "message": err
                    });
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                }
            };

            let users_response = json!({
                "status": "success",
                "data": {
                    "token": token,
                    "user": user
                }
            });
            Ok(Json(users_response))
        }
        Err(err) => {
            if err.to_string().contains("UNIQUE constraint failed") {
                let error_response = json!({
                    "status": "error",
                    "message": "Такий користувач вже існує",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            let error_response = json!({
                "status": "error",
                "message": format!("Помилка БД: {:?}", err)
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}


pub async fn login_user(State(state): State<AppState>,
    Json(payload): Json<LoginUserSchema>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let get_user_by_email = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        if let sqlx::Error::RowNotFound = err {
            let error_response = json!({
                "status": "error",
                "message": "email або пароль невірний"
            });
            return (StatusCode::NOT_FOUND, Json(error_response));
        }
        
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка БД при пошуку емейлу користувача: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;


    // Парсинг хешу для витягнення солі та налаштувань
    let parsed_hash = PasswordHash::new(&get_user_by_email.password_hash)
        .map_err(|_| {
            let error_response = json!({
                "status": "error",
                "message": "Помилка сервера при перервірці хешу"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;


    // Порівняння хешу пароля з фронта з хешованим паролем з БД
    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        let error_response = json!({
            "status": "error",
            "message": "Невірний email або пароль"
        });
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)))
    }

    let role = get_user_by_email.role.as_deref().unwrap_or("customer");

        let token = match generate_jwt_token(get_user_by_email.id, role) {
        Ok(t) => t,
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": err
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let success_response = json!({
        "status": "success",
        "data": {
            "token": token,
            "user":{
                "email": payload.email,
                "role": role
            }
        }
    });

    Ok(Json(success_response))
}


pub fn generate_jwt_token(user_id: i64, role: &str) -> Result<String, String> {
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Помилка генерації часу для токена")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "Помилка сервера: не знайдено JWT_SECRET".to_string())?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ) .map_err(|err| format!("Помилка створення токена: {}", err))
}