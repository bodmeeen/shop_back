use axum::{Json, extract::State};
use crate::{AppState, models::product::Product};


// Хендлер (обробник запиту). Звичайна ф-я яка повертає текст
pub async fn hello_shop() -> &'static str {
    "Бд підключена"
}


pub async fn get_products(State(state): State<AppState>) -> Json<Vec<Product>> {
    // Запит до бази
    let products = sqlx::query_as(
        "SELECT id, title, body, old_price, price, status FROM products"
    )
    .fetch_all(&state.db) // Виконання запиту через пул з'єднань
    .await
    .unwrap_or_else(|_| vec![]); // Якщо помилка, то поверне пустий список

    Json(products)
}