use axum::{
    extract::Request, http::StatusCode, middleware::Next, response::{IntoResponse, Response},
};
use crate::models::users::Claims; 

pub async fn require_admin(
    claims: Claims, 
    request: Request,
    next: Next,
) -> Response {
    
    if claims.role != "admin" {
        // Якщо не адмін - 403 Forbidden
        return StatusCode::FORBIDDEN.into_response();
    }

    next.run(request).await
}