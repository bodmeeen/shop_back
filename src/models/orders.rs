use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Orders {
    pub id: i64,
    pub user_id: i64,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub total_price: i64,
    pub status: String,
    pub created_at: Option<String>
}