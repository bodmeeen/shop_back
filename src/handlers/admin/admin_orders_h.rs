use axum::{Json, extract::{State}};
use crate::{AppState, models::orders::{Order}};


pub async fn get_orders (State(state): State<AppState>) -> Json<Vec<Order>> {
    let orders = sqlx::query_as(
        "SELECT * FROM orders"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    Json(orders)
}   