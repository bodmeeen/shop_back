use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use crate::{AppState}; 

use crate::handlers::{
    product_h::{get_products, create_product, soft_delete_product, get_product_by_id, restore_product, get_products_by_category_id},
    categories_h::{get_categories, create_category, get_category_by_id},
    auth_h::{registration_user, login_user},
    public_orders_h::{create_order, get_order_by_id},
    admin::admin_users_h::{get_users, create_user, delete_user},
    admin::admin_orders_h::{get_orders},
    admin::admin_products_h::{update_product_quantity}
};

pub fn create_router(state: AppState) -> Router {
    // Шар CORS; Потрібно буде змінити з Any
    let cors = CorsLayer::new()
        // Дозволити запити з будь-яких адрес
        .allow_origin(Any)
        // Дозволити ці методи
        .allow_methods([Method::GET, Method::POST, Method:: PUT, Method::PATCH, Method::DELETE])
        // Дозволити будь-які заголовки
        .allow_headers(Any);

    
    // Rate Limiting
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(8)
            .burst_size(60)
            .finish()
            .unwrap(),
    );

    Router::new()

        .route("/api/products", get(get_products).post(create_product))
        .route("/api/products/:id/archive", patch(soft_delete_product))
        .route("/api/products/:id/restore", patch(restore_product))
        .route("/api/get_product_by_id/:id", get(get_product_by_id))
        .route("/api/get_products_by_category_id/:id", get(get_products_by_category_id))
        .route("/api/admin/products/:id/quantity", patch(update_product_quantity))


        .route("/api/categories", get(get_categories).post(create_category))
        .route("/api/category_by_id/:id", get(get_category_by_id))


        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", delete(delete_user))
        .route("/api/users_reg", post(registration_user))
        .route("/api/users_login", post(login_user))


        .route("/api/admin_orders", get(get_orders))
        .route("/api/orders", post(create_order)) 
        .route("/api/orders/:id", get(get_order_by_id))

        .layer(GovernorLayer {
            config: governor_conf,
        })
        .layer(cors)
        .with_state(state)
}