use axum::{Router, middleware, routing::{get, post}};
use sea_orm::DatabaseConnection;
use crate::{auth::auth_middleware, handlers::test_handler::{self, get_test_by_id}};

pub fn get_test_routes() -> Router<DatabaseConnection>{
    Router::new()
    .route("/", post(test_handler::update_and_create_test))
    .route("/:id", get(get_test_by_id))
    .layer(middleware::from_fn(auth_middleware::auth))
}   