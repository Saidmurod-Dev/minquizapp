use axum::{Router, middleware, routing::get};
use sea_orm::DatabaseConnection;

use crate::{auth::auth_middleware, handlers::user_handler::get_user_data};

pub fn get_user_routes() -> Router<DatabaseConnection>{
    Router::new()
    .route("/", get(get_user_data))
    .layer(middleware::from_fn(auth_middleware::auth))
}