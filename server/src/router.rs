use crate::routes;
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

pub fn get_router() -> Router<DatabaseConnection> {
    let router = Router::new()
        .route("/", get(async || "d"))
        .nest("/user", routes::user_routes::get_user_routes())
        .nest("/test", routes::test_routes::get_test_routes())
        .nest("/auth", routes::auth_routes::get_auth_routes());
    router
}
