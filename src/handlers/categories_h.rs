use axum::{Json, extract::{Path, State}, http::StatusCode, response::{IntoResponse}};
use crate::{AppState, models::categories::{Category, CreateCategorySchema}};
use serde_json::json;


pub async fn get_categories(State(state): State<AppState>) -> Json<Vec<Category>> {
    // Запит до бази
    let category = sqlx::query_as(
        "SELECT * FROM categories"
    )
    .fetch_all(&state.db) // Виконання запиту через пул з'єднань
    .await
    // .unwrap_or_else(|_| vec![]); // Якщо помилка, то поверне пустий список
    .unwrap(); // Поки звичайни для нормального виведення помилок
    Json(category)
}


pub async fn create_category(State(state): State<AppState>,
        // отримання даних від користувача в payload
        Json(payload): Json<CreateCategorySchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    
    let result = sqlx::query_as::<_, Category>(
        r#"INSERT INTO categories (title, image_url) 
        VALUES ($1, $2)
        RETURNING *"#,
        )
        .bind(&payload.title)
        .bind(&payload.image_url)

        .fetch_one(&state.db)
        .await;

        match result {
            Ok(category) => {
                let category_response = json!({
                    "status": "success",
                    "data": {
                        "category": category
                    }
                });
                Ok(Json(category_response))
            }
            Err(err) => {
                if err.to_string().contains("UNIQUE constraint failed") {
                    let error_response = json!({
                        "status": "error",
                        "message": "Така категорія вже існує",
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

pub async fn get_category_by_id(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Json<Category> {
    
    let products = sqlx::query_as(
        "SELECT * FROM categories WHERE id = $1"
    )
    .bind(&id)
    .fetch_one(&state.db) // Виконання запиту через пул з'єднань
    .await
    .unwrap();
    Json(products)
}