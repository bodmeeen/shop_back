use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::SocketAddr;

pub mod handlers;
pub mod models;
pub mod telegram;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
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

    let state = AppState { db: pool };

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Сервер запущений за адресою: http://localhost:3000");

    axum::serve(
        listener, 
        // Це для Governor щоб він бачив IP адреси
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}