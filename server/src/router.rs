use axum::{Router, routing::get};
use tower::ServiceBuilder;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use crate::routes::auth_routes;

pub fn get_router() -> Router<DatabaseConnection> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let router = Router::new()
        .layer(ServiceBuilder::new()
            .layer(cors))
    .route("/", get(async || "d"))
    .nest("/auth", auth_routes::get_auth_routes());
    router
}
