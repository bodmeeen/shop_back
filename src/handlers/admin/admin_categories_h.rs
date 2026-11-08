use serde::Deserialize;
use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::{AppState, models::{categories::{Category, CreateCategorySchema}, users::Claims}};

#[derive(Deserialize)]
pub struct UpdateQuantityRequest {
    pub quantity: i64,
}

pub async fn create_category(claims: Claims, State(state): State<AppState>,
        // отримання даних від користувача в payload
        Json(payload): Json<CreateCategorySchema>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    if claims.role != "admin" {
        let error_response = json!({
            "status": "error",
            "message": "Доступ заборонено"
        });
        return Err((StatusCode::FORBIDDEN, Json(error_response)));
    }
    
    
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


// pub async fn soft_delete_product (State(state): State<AppState>,
//         Path(id): Path<i64>,
//         ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

//     let updated_product = sqlx::query_as::<_, Product> (
//         r#"UPDATE products SET status = 'hidden' WHERE id = $1 RETURNING *"#)
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


// pub async fn restore_product (State(state): State<AppState>,
//         Path(id): Path<i64>,
//         ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

//     let updated_product = sqlx::query_as::<_, Product> (
//         r#"UPDATE products SET status = 'active' WHERE id = $1 RETURNING *"#)
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

