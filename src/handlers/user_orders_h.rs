use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use crate::{AppState, models::{orders::Order, users::Claims}};


pub async fn get_orders_by_user_id (
    claims: Claims,
    State(state): State<AppState>
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let orders = sqlx::query_as::<_, Order>(
      "SELECT * FROM orders WHERE user_id = $1 ORDER BY id DESC"
    )
    .bind(claims.sub) // id користувача з токена
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка БД: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    Ok(Json(json!({
        "status": "success",
        "data": orders
    })))
}   