use axum::{Json, body, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use crate::{AppState, models::product::{self, CreateProductSchema, Product}};
use serde_json::json;


// Хендлер (обробник запиту). Звичайна ф-я яка повертає текст
pub async fn hello_shop() -> &'static str {
    "Бд підключена"
}


pub async fn get_products(State(state): State<AppState>) -> Json<Vec<Product>> {
    // Запит до бази
    let products = sqlx::query_as(
        "SELECT * FROM products"
    )
    .fetch_all(&state.db) // Виконання запиту через пул з'єднань
    .await
    // .unwrap_or_else(|_| vec![]); // Якщо помилка, то поверне пустий список
    .unwrap(); // Поки звичайни для нормального виведення помилок
    Json(products)
}


pub async fn create_product(State(state): State<AppState>,
        // отримання даних від користувача в payload
        Json(payload): Json<CreateProductSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    
    let product = sqlx::query_as::<_, Product>(
        r#"INSERT INTO products (title, body, old_price, price, status) 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *"#,
        )
        .bind(&payload.title)
        .bind(&payload.body)
        .bind(&payload.old_price)
        .bind(&payload.price)
        .bind(&payload.status)
        .fetch_one(&state.db)
        .await;

        match product {
            Ok(product) => {
                let product_response = json!({
                    "status": "success",
                    "data": {
                        "product": product
                    }
                });
                Ok(Json(product_response))
            }
            Err(err) => {
                if err.to_string().contains("UNIQUE constraint failed") {
                    let error_response = json!({
                        "status": "error",
                        "message": "Такий товар вже існує",
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


pub async fn delete_product(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let result = sqlx::query_as::<_, Product>(
        r#"DELETE FROM products WHERE id = $1 RETURNING *"#,
        )
        .bind(&id)
        .fetch_one(&state.db)
        .await;

        match result {
            Ok(product) => {
                let product_response = json!({
                    "status": "success",
                    "data": {
                        "product": product
                    }
                });
                Ok(Json(product_response))
            }
            Err(err) => {
                // Потрібно буде знайти як правильно обробити помилку
                // при видаленні в sqlite, та підправити ф-ю
                // if err.to_string().contains("UNIQUE constraint failed") {
                //     let error_response = json!({
                //         "status": "error",
                //         "message": "Помилка при видаленні товару",
                //     });
                //     return Err((StatusCode::CONFLICT, Json(error_response)));
                // }
                let error_response = json!({
                    "status": "error",
                    "message": format!("Помилка БД: {:?}", err)
                });
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
            }
        }
}