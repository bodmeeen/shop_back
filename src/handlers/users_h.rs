use axum::{Json, extract::{State}, http::StatusCode, response::{IntoResponse}};
use sqlx::error;
use crate::{AppState, models::users::{LoginUserSchema, RegisterUserSchema, User}};
use serde_json::json;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher
};

pub async fn create_user_hash(State(state): State<AppState>,
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
        .bind("user")
        .fetch_one(&state.db)
        .await;

        match result {
            Ok(user) => {
                let users_response = json!({
                    "status": "success",
                    "data": {
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


        let success_response = json!({
        "status": "success",
        "data": {
            "isValid" : "yes"
        }
    });

    Ok(Json(success_response))
}