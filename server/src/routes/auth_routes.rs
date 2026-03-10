use axum::{
    Router,
    routing::post
};
use sea_orm::DatabaseConnection;
use crate::handlers::{
    user_handler::register_user
};

pub fn get_auth_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/register", post(register_user))
    
}