use axum::{Json, extract::{Path, State}, http::StatusCode, response::{IntoResponse}};
use crate::{AppState, models::users::{User, CreateUserSchema, /*UpdateUserSchema*/}};
use serde_json::json;

pub async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as(
        "SELECT * FROM users"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    Json(users)
}

pub async fn create_user(State(state): State<AppState>,
        Json(payload): Json<CreateUserSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    
    let result = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (first_name, last_name, role, email, password_hash, phone_number) 
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *"#,
        )
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.role)
        .bind(&payload.email)
        .bind(&payload.password_hash)
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

pub async fn delete_user(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let result = sqlx::query_as::<_, User> (
        r#"DELETE FROM users WHERE id = $1 RETURNING *"#)
        .bind(&id)
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
        Err(sqlx::Error::RowNotFound) => {
            let error_response = json!({
                "status": "error",
                "message": format!("Користувач з ID: {} не знайдено", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": format!("Помилка БД: {:?}", err)
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}