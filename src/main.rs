use axum::routing::{delete, post};
use axum::{routing::get, Router, extract::State};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

mod db;
mod routes;
mod models;
use routes::handlers::hello_shop;
use routes::handlers::{get_products, create_product};

use crate::routes::handlers::delete_product;

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

    let app = Router::new()
        .route("/", get(hello_shop))
        .route("/api/products", get(get_products))
        .route("/api/products", post(create_product))
        .route("/api/products/:id", delete(delete_product))
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

