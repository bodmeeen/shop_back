use axum::{Json, extract::{Path, State}, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::{AppState, models:: users::{UpdateUserSchema, User},};

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": "Користувача не знайдено" }))
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "user": user
        }
    })))
}

    // Ф-я для зміни даних користувача в кабінеті користувача
pub async fn update_user_data(State(state): State<AppState>,
        Path(id): Path<i64>,
        Json(payload): Json<UpdateUserSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        
    let query_result = sqlx::query_as::<_, User> (
        r#"SELECT * FROM users WHERE id = $1"#)
            .bind(&id)
            .fetch_one(&state.db)
            .await;
        
    let existing_user = match query_result {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Користувача з ID: {} не знайдено", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Помилка БД: {:?}",err)
                })),
            ));
        }
    };

    let new_first_name = payload.first_name.clone().or(Some(existing_user.first_name.clone()));
    let new_last_name = payload.last_name.clone().or(existing_user.last_name.clone());
    let new_email = payload.email.clone().unwrap_or(existing_user.email.clone());
    let new_phone_number = payload.phone_number.clone().or(existing_user.phone_number.clone());

    let updated_user = sqlx::query_as::<_, User> (
        r#"UPDATE users SET first_name = $1, last_name = $2, email = $3, phone_number = $4 WHERE id = $5 RETURNING *"#)
        .bind(new_first_name)
        .bind(new_last_name)
        .bind(new_email)
        .bind(new_phone_number)
        .bind(&id)
        .fetch_one(&state.db)
        .await

        .map_err(|err| {
            (
            StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Помилка БД: {:?}", err)
                })),
            )
        })?;

        let response = json!({
            "status": "success",
            "data": json!({
                "user": updated_user
            })
        });

        Ok(Json(response))

}