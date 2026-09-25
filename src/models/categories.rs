use serde::{Deserialize, Serialize};
// схеми
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct Categories {
    pub id: i64,
    pub title: String,
    pub image_url: String
}

#[derive(Deserialize)]
pub struct CreateCategorySchema {
    pub title: String,
    pub image_url: String
}