use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use crate::routes::auth_routes;

pub fn get_router() -> Router<DatabaseConnection> {
    let router = Router::new()
    .route("/", get(async || "d"))
    .nest("/auth", auth_routes::get_auth_routes());
    router
}
