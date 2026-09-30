use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Debug, FromRow)]
pub struct Order {
    pub id: i64,
    pub user_id: Option<i64>,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_phone: String,
    pub delivery_info: String,
    pub total_price: i64,
    pub status: String,
    pub created_at: Option<String>
}

#[derive(Serialize, Debug, FromRow)]
pub struct OrderItem {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub price_at_purchase: i64
}



#[derive(Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: i64,
    pub quantity: i64,
    pub price_at_purchase: i64
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: Option<i64>,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_phone: String,
    pub delivery_info: String,
    pub total_price: i64,
    pub items: Vec<CreateOrderItemRequest>
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub order: Order,
    pub items: Vec<OrderItem>
}


