use std::env;

use serde_json::json;

pub async fn send_telegram_notification(order_text: &str) {

    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("Не знайдено TELEGRAM_BOT_TOKEN в .env");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("Не знайдено TELEGRAM_CHAT_ID в .env");
    
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let client = reqwest::Client::new();
    
    // Формування запиту та його відправлення
    let result = client.post(&url)
        .json(&json!({
            "chat_id": chat_id,
            "text": order_text,
            "parse_mode": "HTML" // Дозволяє робити текст жирним за допомогою <b> </b>
        }))
        .send()
        .await;

    match result {
        Ok(_) => println!("Працює"),
        Err(e) => eprintln!("Не працює. err: {}", e),
    }
}