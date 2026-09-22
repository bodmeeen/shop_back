use sqlx::SqlitePool;

pub async fn setup_database(pool: &SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            old_price INTEGER,
            price INTEGER NOT NULL,
            status TEXT NOT NULL,
            created_at DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await
    .unwrap();

    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if count == 0 {
        // якщо написати query!, то компілятор зупиняється та перевіряє чи існує таблиця
        sqlx::query( 
            "INSERT INTO products (title, body, price, status) VALUES
            ('Плюшевий ведмідь', 'Мяка іграшка', 750, 'В наявності'),
            ('Плакат', 'Плакат', 220, 'В наявності')"
        )
        .execute(pool)
        .await
        .unwrap();
    println!("Сидінг успішний");
    }
}