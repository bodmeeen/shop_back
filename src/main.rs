use axum::{routing::{get, delete, post, patch}, Router, http::Method};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use tower_http::cors::{Any, CorsLayer};


mod db;
mod handlers;
mod models;
use handlers::product_h::hello_shop;
use handlers::product_h::{get_products, create_product};

use crate::handlers::product_h::{delete_product, update_product};

#[derive(Clone)]
// SqlitePool працює як Arc, тож його не потрібно
// додатково використовувати
struct AppState {
    db: SqlitePool,
}

#[tokio::main] // Макрос
async fn main() {
    // Вказування що читати змінні потрібно і з .env файлу
    // ok() - якщо відкриття .env не вдалось, то пропустити >
    // > видає Option замість Result
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Не знайдено DATABASE_URL");

    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Успішно під'єднано до БД");
            pool
        }
        Err(err) => {
            println!("Помилка при підключенні до БД: {}", err);
            std::process::exit(1);
        }
    };

    db::seeding::setup_database(&pool).await;

    let state = AppState { db: pool.clone() };

    // Шар CORS
    let cors = CorsLayer::new()
        // Дозволити запити з будь-яких адрес
        .allow_origin(Any)
        // Дозволити ці методи
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // Дозволити будь-які заголовки
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(hello_shop))
        .route("/api/products", get(get_products))
        .route("/api/products", post(create_product))
        .route("/api/products/:id", delete(delete_product))
        .route("/api/products/:id", patch(update_product))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Сервер запущений за адресою: http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

//     // Роутер; якщо користувач зайшов на головну сторінку, то для
//     // нього виконується ф-я hello_shop
//     let app = Router::new()
//         .route("/", get(hello_shop));

//     // Підключення до порту 3000 локально
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

//     println!("Сервер запущений за адресою: http://localhost:3000");

//     // serve це безкінечний цикл, сервер працює поки не перервати 
//     // його роботу через термінал
//     axum::serve(listener, app).await.unwrap();
// }

