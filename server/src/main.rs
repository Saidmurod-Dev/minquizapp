use axum::Router;
use sea_orm::{DbErr};
use dotenv::dotenv;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

mod router;
mod db;
mod routes;
mod auth;
mod handlers;
mod entity;
mod hash;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let db = db::connect_db().await?;

    let app = Router::new()
    .merge(router::get_router())
    .with_state(db);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
