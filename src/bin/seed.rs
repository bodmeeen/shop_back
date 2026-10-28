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
    let cat_constructors = sqlx::query!("INSERT INTO categories (title, image_url) VALUES ('Конструктори', '/images/cats/constructors.jpg') RETURNING id").fetch_one(&pool).await?.id;
    let cat_puzzles = sqlx::query!("INSERT INTO categories (title, image_url) VALUES ('Пазли', '/images/cats/puzzles.jpg') RETURNING id").fetch_one(&pool).await?.id;


    // Товари 
    let prod_1 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Величезний ведмідь Балу', ?, 'М''який плюшевий ведмідь, 120 см, карамельний колір. Ідеальний для подарунка.', 150000, 120000, 2, 'in_stock') RETURNING id",
        cat_bears
    ).fetch_one(&pool).await?.id;

    let prod_2 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Зайчик Вухастик', ?, 'Ніжний зайчик для сну, 30 см, гіпоалергенний матеріал.', NULL, 65000, 2, 'in_stock') RETURNING id",
        cat_bunnies
    ).fetch_one(&pool).await?.id;

    let prod_3 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Собачка, що гавкає', ?, 'Інтерактивна собачка, ходить і видає звуки. Живлення від батарейок.', 110000, 95000, 2, 'in_stock') RETURNING id",
        cat_interactive
    ).fetch_one(&pool).await?.id;

    let prod_4 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Набір конструктора ''Замок''', ?, 'Конструктор Lego', 200000, 180000, 2, 'in_stock') RETURNING id",
        cat_constructors
    ).fetch_one(&pool).await?.id;

    let prod_5 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Пазл ''Картина''', ?, 'Пазл розміром 1000x1250 см', 125000, 115000, 2, 'in_stock') RETURNING id",
        cat_puzzles
    ).fetch_one(&pool).await?.id;

    let prod_6 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Ведмедик Паддінгтон', ?, 'Класичний плюшевий ведмедик у синьому пальто та червоному капелюсі. Висота 40 см.', NULL, 85000, 2, 'in_stock') RETURNING id",
        cat_bears
    ).fetch_one(&pool).await?.id;

    let prod_7 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Панда По', ?, 'Чорно-біла плюшева панда, дуже м''яка на дотик. Ідеально підходить для обіймів.', 100000, 89000, 2, 'in_stock') RETURNING id",
        cat_bears
    ).fetch_one(&pool).await?.id;

    let prod_8 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Зайчиха Міла', ?, 'Милий зайчик у рожевій сукні, з довгими вушками. Прекрасний подарунок для дівчинки.', 75000, 60000, 2, 'in_stock') RETURNING id",
        cat_bunnies
    ).fetch_one(&pool).await?.id;

    let prod_9 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Заєць-стрибунець', ?, 'Маленький пухнастий зайчик сірого кольору. Компактний розмір, зручно брати з собою.', NULL, 45000, 2, 'in_stock') RETURNING id",
        cat_bunnies
    ).fetch_one(&pool).await?.id;

    let prod_10 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Кіт, що муркоче', ?, 'Сенсорна іграшка. Реагує на погладжування, муркоче та махає хвостом.', 130000, 115000, 2, 'in_stock') RETURNING id",
        cat_interactive
    ).fetch_one(&pool).await?.id;

    let prod_11 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Робот-динозавр Рекс', ?, 'Ходить, гарчить, очі світяться. Керується за допомогою пульта.', 250000, 210000, 2, 'in_stock') RETURNING id",
        cat_interactive
    ).fetch_one(&pool).await?.id;

    let prod_12 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Магнітний конструктор', ?, 'Набір з 50 яскравих магнітних деталей для розвитку просторової уяви.', 180000, 155000, 2, 'in_stock') RETURNING id",
        cat_constructors
    ).fetch_one(&pool).await?.id;

    let prod_13 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Дерев''яне містечко', ?, 'Екологічний конструктор з натурального дерева, покритий безпечними фарбами.', NULL, 95000, 2, 'in_stock') RETURNING id",
        cat_constructors
    ).fetch_one(&pool).await?.id;

    let prod_14 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('Пазл \"Космос\"', ?, 'Пазл на 500 елементів. Готова картина світиться в темряві!', 65000, 50000, 2, 'in_stock') RETURNING id",
        cat_puzzles
    ).fetch_one(&pool).await?.id;

    let prod_15 = sqlx::query!(
        "INSERT INTO products (title, category_id, body, old_price, price, quantity, status) VALUES ('3D-пазл \"Корабель\"', ?, 'Об''ємний пазл для складання дерев''яного вітрильника. Клей не потрібен.', 150000, 135000, 2, 'in_stock') RETURNING id",
        cat_puzzles
    ).fetch_one(&pool).await?.id;



    // Картинки до товарів
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/balu_main.jpg', 1)", prod_1).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/bunny_main.jpg', 1)", prod_2).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/dog_main.jpg', 1)", prod_3).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/const_catle_lego.jpg', 1)", prod_4).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/puzzle_1000x1250.jpg', 1)", prod_5).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/paddington_main.jpg', 1)", prod_6).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/panda_main.jpg', 1)", prod_7).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/bunny_mila_main.jpg', 1)", prod_8).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/bunny_jump_main.jpg', 1)", prod_9).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/cat_purr_main.jpg', 1)", prod_10).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/dino_rex_main.jpg', 1)", prod_11).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/magnetic_const_main.jpg', 1)", prod_12).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/wood_town_main.jpg', 1)", prod_13).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/puzzle_space_main.jpg', 1)", prod_14).execute(&pool).await?;
    sqlx::query!("INSERT INTO product_images (product_id, image_url, is_main) VALUES (?, '/images/products/puzzle_ship_main.jpg', 1)", prod_15).execute(&pool).await?;

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