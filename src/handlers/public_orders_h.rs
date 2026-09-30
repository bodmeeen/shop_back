use axum::{Json, extract::{Path, State}, http::StatusCode, response::{IntoResponse}};
use crate::{AppState, models::orders::{CreateOrderRequest, Order, OrderItem}};
use serde_json::json;

// приймає дані, відкриває транзакцію, зберігає клієнта та товари, підтвердження
pub async fn create_order (State(state): State<AppState>,
        // отримання даних від користувача в payload
        Json(payload): Json<CreateOrderRequest>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    // Відкриття транзакції
    let mut tx = state.db.begin().await.map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка БД при старті транзакції: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Створення замовлення і збирання id
    // query_scalar - бо потрібна тільки одна цифра а не ціла структура
    let order_id = sqlx::query_scalar::<_, i64>(
        r#"INSERT INTO orders (customer_first_name, customer_last_name, customer_phone, delivery_info, total_price, status) 
        VALUES ($1, $2, $3, $4, $5, 'new')
        RETURNING id"#
    )
    .bind(&payload.customer_first_name)
    .bind(&payload.customer_last_name)
    .bind(&payload.customer_phone)
    .bind(&payload.delivery_info)
    .bind(&payload.total_price)
    // Передавання транзакції, а не &state.db
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка при збереженні замовлення: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Збереження товарів з кошика
    for item in payload.items {
        sqlx::query(
            r#"INSERT INTO order_items (order_id, product_id, quantity, price_at_purchase) 
            VALUES ($1, $2, $3, $4)"#
        )
        .bind(order_id) // id який отримано з бази
        .bind(item.product_id)
        .bind(item.quantity)
        .bind(item.price_at_purchase)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "error",
                "message": format!("Помилка при збереженні товару: {:?}", err)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    }

    // Підтвердження транзакції (збереження в БД)
    tx.commit().await.map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка при підтвердженні транзакції: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;


    let success_response = json!({
        "status": "success",
        "data": {
            "order_id": order_id
        }
    });

    // 201 Created і JSON
    Ok((StatusCode::CREATED, Json(success_response)))
}



// отримує id, шукає замовлення та товари, об'єднує це все
pub async fn get_order_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    // Діставання замовлення
    let order = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        // Якщо рядка немає, то 404
        if let sqlx::Error::RowNotFound = err {
            let error_response = json!({
                "status": "error",
                "message": "Замовлення не знайдено"
            });
            return (StatusCode::NOT_FOUND, Json(error_response));
        }
        
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка БД при отриманні замовлення: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Діставання всіх товарів
    let items = sqlx::query_as::<_, OrderItem>(
        "SELECT * FROM order_items WHERE order_id = $1"
    )
    .bind(id)
    // fetch_all не дає RowNotFound, якщо пусто то буде пустий масив
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Помилка БД при отриманні товарів: {:?}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Пакування
    let success_response = json!({
        "status": "success",
        "data": {
            "order": order,
            "items": items
        }
    });

    Ok(Json(success_response))
}