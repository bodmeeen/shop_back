use axum::{Json, extract::{State}, http::StatusCode, response::{IntoResponse}};
use crate::{AppState, models::users::{User, CreateUserPassSchema}};
use serde_json::json;
use argon2::{
    password_hash::{PasswordHasher},
    Argon2
};

pub async fn create_user_hash(State(state): State<AppState>,
        Json(payload): Json<CreateUserPassSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(payload.password.as_bytes())
        .expect("Помилка хешування пароля")
        .to_string();


    // Це для логіну; Розпарсити хеш та порівняти з ним пароль; 
    // assert! потрібно буде видалити бо він крашить все 
    // let parsed_hash = PasswordHash::new(&password_hash)?;
    // assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());

    let result = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (first_name, last_name, role, email, password_hash, phone_number) 
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *"#,
        )
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.role)
        .bind(&payload.email)
        .bind(&hashed_password)
        .bind(&payload.phone_number)
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