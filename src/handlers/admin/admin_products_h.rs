use serde::Deserialize;
use axum::{Json, extract::{Path, State}, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
pub struct UpdateQuantityRequest {
    pub quantity: i64,
}

pub async fn update_product_quantity(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateQuantityRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    // Перевірка щоб з фронта не прийшов мінус
    if payload.quantity < 0 {
        let error_response = json!({
            "status": "error",
            "message": "Кількість не може бути меншою за нуль"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let result = sqlx::query!(
        "UPDATE products SET quantity = $1 WHERE id = $2",
        payload.quantity,
        id
    )
    .execute(&state.db)
    .await
    .map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка оновлення кількості: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if result.rows_affected() == 0 {
        let error_response = json!({
            "status": "error",
            "message": "Товар не знайдено"
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok((StatusCode::OK, Json(json!({"status": "success"}))))
}