use axum::{Json, extract::{State}, response::{IntoResponse}};
use crate::{AppState, models::orders::{Order}};

// окремо для таблиці orders
pub async fn get_orders (State(state): State<AppState>) -> Json<Vec<Order>> {
    let orders = sqlx::query_as(
        "SELECT * FROM orders"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    Json(orders)
}   