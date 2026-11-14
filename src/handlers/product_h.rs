use axum::{Json, extract::{Path, State}, response::IntoResponse};
use reqwest::StatusCode;
use crate::{AppState, models::products::{Product}};

// Хендлер (обробник запиту). Звичайна ф-я яка повертає текст

pub async fn get_active_products(State(state): State<AppState>) -> Json<Vec<Product>> {
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
        WHERE p.status = 'active'
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    Json(products)
}

pub async fn get_product_by_id(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Json<Product> {
    
    let product = sqlx::query_as::<_, Product>(
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
        WHERE p.id = $1
        "#
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await
    .unwrap();
    Json(product)
}

pub async fn get_products_by_category_id(State(state): State<AppState>,
        Path(id): Path<i64>,
        ) -> Json<Vec<Product>> {
    
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
        WHERE p.category_id = $1 AND p.status = 'active'
        "#
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|_| vec![]);
    Json(products)
}

// Отримання схожих товарів ( з тієї ж категорії, крім поточного)
pub async fn get_similar_products(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
        WHERE p.category_id = (
            SELECT category_id FROM products WHERE id = $1
        )
          AND p.id != $1
          AND p.status = 'active'
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