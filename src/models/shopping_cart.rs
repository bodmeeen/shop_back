use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct ShoppingCart {
    pub user_id: i64,
    pub product_id: i64,
    pub amount: i64,
    pub created_at: Option<String>
}