use std::sync::Arc;

use std::path::PathBuf;

use axum::{
    Router, http::Method, middleware, routing::{get, patch, post},
};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use crate::{AppState, handlers::{middleware::require_admin, product_h::get_similar_products, users_h::{get_user_by_id, update_user_data}}}; 

use crate::handlers::{
    product_h::{get_active_products, get_product_by_id, get_products_by_category_id},
    categories_h::{get_categories, get_category_by_id},
    auth_h::{registration_user, login_user},
    orders_h::{create_order, get_order_by_id},
    user_orders_h::{get_orders_by_user_id},

    admin::admin_users_h::{get_users, create_user, delete_user},
    admin::admin_orders_h::{get_orders},
    admin::admin_products_h::{get_products, update_product_quantity, create_product, soft_delete_product, restore_product},
    admin::admin_categories_h::{create_category},
};

pub fn create_router(state: AppState) -> Router {
    // Шар CORS; Потрібно буде змінити з Any
    let cors = CorsLayer::new()
        // Дозволити запити з будь-яких адрес
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    
    // Rate Limiting    
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(8)
            .burst_size(60)
            .finish()
            .unwrap(),
    );


    let admin_routes = Router::new()
        .route("/api/admin/products", get(get_products).post(create_product))
        .route("/api/admin/orders", get(get_orders))
        .route("/api/admin/products/:id/quantity", patch(update_product_quantity))
        .route_layer(middleware::from_fn(require_admin));

    let products_routes = Router::new()
        .route("/api/products", get(get_active_products))
        .route("/api/products/:id", get(get_product_by_id))
        .route("/api/products/category/:id", get(get_products_by_category_id))
        .route("/api/products/:id/archive", patch(soft_delete_product))
        .route("/api/products/:id/restore", patch(restore_product))
        .route("/api/products/:id/similar", get(get_similar_products));

    let users_routes = Router::new()
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user_by_id).put(update_user_data).delete(delete_user))
        .route("/api/users/registration", post(registration_user))
        .route("/api/users/login", post(login_user));
    
    let categories_routes = Router::new()
        .route("/api/orders", post(create_order).get(get_orders_by_user_id)) 
        .route("/api/orders/:id", get(get_order_by_id));

    let orders_routes = Router::new()
        .route("/api/categories", get(get_categories).post(create_category))
        .route("/api/categories/:id", get(get_category_by_id));

    let images_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public/images");
    let static_routes = Router::new().nest_service("/images", ServeDir::new(images_dir));

    Router::new()
        .merge(products_routes)
        .merge(users_routes)
        .merge(categories_routes)
        .merge(orders_routes)
        .merge(admin_routes)
        .merge(static_routes)
        .layer(GovernorLayer { config: governor_conf })
        .layer(cors)
        .with_state(state)
}

