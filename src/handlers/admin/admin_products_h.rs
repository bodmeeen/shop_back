use serde::Deserialize;
use axum::{Json, extract::{Path, State}, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::{AppState, models::{products::{CreateProductSchema, Product}, users::Claims}};

#[derive(Deserialize)]
pub struct UpdateQuantityRequest {
    pub quantity: i64,
}

pub async fn get_products(State(state): State<AppState>) -> Json<Vec<Product>> {
    let products = sqlx::query_as::<_, Product>(
        r#"
        SELECT
            p.id,
            p.title,
            p.category_id,
            p.body,
            p.old_price,
            p.price,
            p.quantity,
            p.status,
            p.is_active,
            p.created_at,
            pi.image_url AS image_url
        FROM products AS p
        LEFT JOIN product_images AS pi
            ON pi.product_id = p.id AND pi.is_main = 1
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    Json(products)
}

pub async fn update_product_quantity(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateQuantityRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    if claims.role != "admin" {
        let error_response = json!({
            "status": "error",
            "message": "Доступ заборонено"
        });
        return Err((StatusCode::FORBIDDEN, Json(error_response)));
    }
    
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

pub async fn create_product(
    claims: Claims,
    State(state): State<AppState>,
        // отримання даних від користувача в payload
        Json(payload): Json<CreateProductSchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    if claims.role != "admin" {
        let error_response = json!({
            "status": "error",
            "message": "Доступ заборонено"
        });
        return Err((StatusCode::FORBIDDEN, Json(error_response)));
    }
    
    let result = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (title, category_id, body, old_price, price, quantity, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id,
            title,
            category_id,
            body,
            old_price,
            price,
            quantity,
            status,
            is_active,
            created_at,
            NULL AS image_url
        "#,
        )
        .bind(&payload.title)
        .bind(&payload.category_id)
        .bind(&payload.body)
        .bind(&payload.old_price)
        .bind(&payload.price)
        .bind(&payload.quantity)
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


pub async fn soft_delete_product (State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let updated_product = sqlx::query_as::<_, Product> (
        r#"
        UPDATE products
        SET status = 'hidden'
        WHERE id = $1
        RETURNING
            id,
            title,
            category_id,
            body,
            old_price,
            price,
            quantity,
            status,
            is_active,
            created_at,
            NULL AS image_url
        "#)
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
                "product": updated_product
            })
        });
        Ok(Json(response))
}


pub async fn restore_product (State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let updated_product = sqlx::query_as::<_, Product> (
        r#"
        UPDATE products
        SET status = 'active'
        WHERE id = $1
        RETURNING
            id,
            title,
            category_id,
            body,
            old_price,
            price,
            quantity,
            status,
            is_active,
            created_at,
            NULL AS image_url
        "#)
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
                "product": updated_product
            })
        });
        Ok(Json(response))
}












        // Ф-я для повного видалення товару з БД
// pub async fn delete_product(State(state): State<AppState>,
//         Path(id): Path<i64>,
//         ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
//     let result = sqlx::query_as::<_, Product> (
//         r#"DELETE FROM products WHERE id = $1 RETURNING *"#)
//         .bind(&id)
//         .fetch_one(&state.db)
//         .await;

//         match result {
//             Ok(product) => {
//             let product_response = json!({
//                 "status": "success",
//                 "data": {
//                     "product": product
//                 }
//             });
//             Ok(Json(product_response))
//             }
//         Err(sqlx::Error::RowNotFound) => {
//             let error_response = json!({
//                 "status": "error",
//                 "message": format!("Товар з ID: {} не знайдено", id)
//             });
//             Err((StatusCode::NOT_FOUND, Json(error_response)))
//         }
//         Err(err) => {
//             let error_response = json!({
//                 "status": "error",
//                 "message": format!("Помилка БД: {:?}", err)
//             });
//             Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
//         }
//     }
// }


// pub async fn update_product(State(state): State<AppState>,
//         Path(id): Path<i64>,
//         Json(payload): Json<UpdateProductSchema>,
//         ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        
//     let query_result = sqlx::query_as::<_, Product> (
//         r#"SELECT * FROM products WHERE id = $1"#/*, &id*/)
//             .bind(&id)
//             .fetch_one(&state.db)
//             .await;
        
//     let existing_product = match query_result {
//         Ok(product) => product,
//         Err(sqlx::Error::RowNotFound) => {
//             let error_response = serde_json::json!({
//                 "status": "error",
//                 "message": format!("Товар з ID: {} не знайдено", id)
//             });
//             return Err((StatusCode::NOT_FOUND, Json(error_response)));
//         }
//         Err(err) => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({
//                     "status": "error",
//                     "message": format!("Помилка БД: {:?}",err)
//                 })),
//             ));
//         }
//     };

//     let new_title = payload.title.as_ref().unwrap_or(&existing_product.title);
//     let new_body = payload.body.as_ref().unwrap_or(&existing_product.body);
//     // let new_old_price = payload.old_price.unwrap_or(existing_product.old_price);
//     let new_price = payload.price.unwrap_or(existing_product.price);
//     let new_status = payload.status.as_ref().unwrap_or(&existing_product.status);

//     let updated_product = sqlx::query_as::<_, Product> (
//         r#"UPDATE products SET title = $1, body = $2, price = $3, status = $4 WHERE id = $5 RETURNING *"#)
//         .bind(new_title)
//         .bind(new_body)
//         // .bind(new_old_price)
//         .bind(new_price)
//         .bind(new_status)
//         .bind(&id)
//         .fetch_one(&state.db)
//         .await

//         .map_err(|err| {
//             (
//             StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({
//                     "status": "error",
//                     "message": format!("Помилка БД: {:?}", err)
//                 })),
//             )
//         })?;

//         let response = json!({
//             "status": "success",
//             "data": json!({
//                 "product": updated_product
//             })
//         });

//         Ok(Json(response))

// }