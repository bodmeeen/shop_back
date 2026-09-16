use axum::{routing::get, Router, extract::State};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

mod db;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main] // Макрос
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Не знайдено DATABASE_URL");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Не вдалося підключитись до бд");

    db::seeding::setup_database(&pool).await;

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(hello_shop))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Сервер запущений за адресою: http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

// Хендлер (обробник запиту). Звичана ф-я яка повертає текст
async fn hello_shop() -> &'static str {
    "Бд підключена"
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

