use axum::{Json, extract::{Path, State}, http::StatusCode, response::{IntoResponse}};
use crate::{AppState, models::product::{Product, CreateProductSchema, UpdateProductSchema}};
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
    
    
    let result = sqlx::query_as::<_, Product>(
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
    
    let result = sqlx::query_as::<_, Product> (
        r#"DELETE FROM products WHERE id = $1 RETURNING *"#)
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

pub async fn update_product(State(state): State<AppState>,
        Path(id): Path<i64>,
        Json(payload): Json<UpdateProductSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        
    let query_result = sqlx::query_as::<_, Product> (
        r#"SELECT * FROM products WHERE id = $1"#/*, &id*/)
            .bind(&id)
            .fetch_one(&state.db)
            .await;
        
    let existing_product = match query_result {
        Ok(product) => product,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Товар з ID: {} не знайдено", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Помила БД{:?}",err)
                })),
            ));
        }
    };

    let new_title = payload.title.as_ref().unwrap_or(&existing_product.title);
    let new_body = payload.body.as_ref().unwrap_or(&existing_product.body);
    // let new_old_price = payload.old_price.unwrap_or(existing_product.old_price);
    let new_price = payload.price.unwrap_or(existing_product.price);
    let new_status = payload.status.as_ref().unwrap_or(&existing_product.status);

    let updated_product = sqlx::query_as::<_, Product> (
        r#"UPDATE products SET title = $1, body = $2, price = $3, status = $4 WHERE id = $5 RETURNING *"#)
        .bind(new_title)
        .bind(new_body)
        // .bind(new_old_price)
        .bind(new_price)
        .bind(new_status)
        .bind(&id)
        .fetch_one(&state.db)
        .await

        .map_err(|err| {
            (
            StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Помилка запиту :{:?}", err)
                })),
            )
        })?;

        let response = json!({
            "status": "success",
            "data": json!({
                "product": updated_product
            })
        });

        Ok(Json(response))

}