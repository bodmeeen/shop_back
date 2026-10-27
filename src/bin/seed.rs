use sqlx::{sqlite::SqlitePoolOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://shop.db".to_string());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // Очищення БД
    sqlx::query!("DELETE FROM order_items").execute(&pool).await?;
    sqlx::query!("DELETE FROM orders").execute(&pool).await?;
    sqlx::query!("DELETE FROM shopping_cart").execute(&pool).await?;
    sqlx::query!("DELETE FROM wish_list").execute(&pool).await?;
    sqlx::query!("DELETE FROM product_images").execute(&pool).await?;
    sqlx::query!("DELETE FROM products").execute(&pool).await?;
    sqlx::query!("DELETE FROM categories").execute(&pool).await?;
    sqlx::query!("DELETE FROM users").execute(&pool).await?;

    // Категорії 
    let cat_bears = sqlx::query!("INSERT INTO categories (title, image_url) VALUES ('Плюшеві ведмедики', '/images/cats/bears.jpg') RETURNING id").fetch_one(&pool).await?.id;
    let cat_bunnies = sqlx::query!("INSERT INTO categories (title, image_url) VALUES ('Зайчики', '/images/cats/bunnies.jpg') RETURNING id").fetch_one(&pool).await?.id;
    let cat_interactive = sqlx::query!("INSERT INTO categories (title, image_url) VALUES ('Інтерактивні іграшки', '/images/cats/interactive.jpg') RETURNING id").fetch_one(&pool).await?.id;

    // Товари 
    let prod_1 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, status) VALUES ('Величезний ведмідь Балу', ?, 'М''який плюшевий ведмідь, 120 см, карамельний колір. Ідеальний для подарунка.', 150000, 120000, 'in_stock') RETURNING id",
        cat_bears
    ).fetch_one(&pool).await?.id;

    let prod_2 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, status) VALUES ('Зайчик Вухастик', ?, 'Ніжний зайчик для сну, 30 см, гіпоалергенний матеріал.', NULL, 65000, 'in_stock') RETURNING id",
        cat_bunnies
    ).fetch_one(&pool).await?.id;

    let prod_3 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, status) VALUES ('Собачка, що гавкає', ?, 'Інтерактивна собачка, ходить і видає звуки. Живлення від батарейок.', 110000, 95000, 'low_stock') RETURNING id",
        cat_interactive
    ).fetch_one(&pool).await?.id;

    // Картинки до товарів
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/balu_main.jpg', 1)", prod_1).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/bunny_main.jpg', 1)", prod_2).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/dog_main.jpg', 1)", prod_3).execute(&pool).await?;

    // Користувачі
    let user_id = sqlx::query!(
        "INSERT INTO users (first_name, last_name, role, email, password_hash, phone_number) VALUES ('Максим', 'Покупець', 'customer', 'max@example.com', 'hashed_pass_123', '+380991234567') RETURNING id"
    ).fetch_one(&pool).await?.id;

    // Замовлення
    let order_id = sqlx::query!(
        "INSERT INTO orders (user_id, customer_first_name, customer_last_name, customer_phone, delivery_info, total_price, status) VALUES (?, 'Максим', 'Покупець', '+380991234567', 'м. Чернівці, Відділення 123', 240000, 'processing') RETURNING id",
        user_id
    ).fetch_one(&pool).await?.id;

    sqlx::query!("INSERT INTO order_items (order_id, product_id, quantity, price_at_purchase) VALUES (?, ?, 2, 1200)", order_id, prod_1).execute(&pool).await?;

    Ok(())
}