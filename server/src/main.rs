use axum::Router;
use dotenv::dotenv;
use sea_orm::DbErr;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod db;
mod entity;
mod handlers;
mod hash;
mod router;
mod routes;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let db = db::connect_db().await?;
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .merge(router::get_router())
        .layer(cors)
        .with_state(db);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
