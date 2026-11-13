use axum::{Json, extract::{Path, State}, response::IntoResponse};
use reqwest::StatusCode;
use crate::{AppState, models::products::{Product}};

// Хендлер (обробник запиту). Звичайна ф-я яка повертає текст

pub async fn get_active_products(State(state): State<AppState>) -> Json<Vec<Product>> {
    // Запит до бази
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE status = 'active'"
    )
    .fetch_all(&state.db) // Виконання запиту через пул з'єднань
    .await
    // .unwrap_or_else(|_| vec![]); // Якщо помилка, то поверне пустий список
    .unwrap(); // Поки звичайний для нормального виведення помилок, потрібно буде замінити на нормальне
    Json(products)
}

pub async fn get_product_by_id(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Json<Product> {
    
    let products = sqlx::query_as(
        "SELECT * FROM products WHERE id = $1"
    )
    .bind(&id)
    .fetch_one(&state.db) // Виконання запиту через пул з'єднань
    .await
    // .unwrap_or_else(|_| vec![]); // Якщо помилка, то поверне пустий список
    .unwrap(); // Поки звичайний для нормального виведення помилок
    Json(products)
}

pub async fn get_products_by_category_id(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Json<Vec<Product>> {
    
    let products = sqlx::query_as(
        "SELECT * FROM products WHERE category_id = $1 AND status = 'active'"
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|_| vec![]); // Якщо помилка, то поверне пустий список
    // .unwrap(); // Поки звичайний для нормального виведення помилок
    Json(products)
}

// Отримання схожих товарів ( з тієї ж категорії, крім поточного)
pub async fn get_similar_products(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT * FROM products 
        WHERE category_id = (SELECT category_id FROM products WHERE id = $1)
          AND id != $1 
          AND status = 'active'
        ORDER BY RANDOM() 
        LIMIT 4
        "#
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Помилка БД: {:?}", err)
            })),
        )
    })?;

    Ok(Json(products))
}