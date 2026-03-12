use std::env;

use axum::{
    body::Body, http::Request, middleware::Next, response::Response
};
use crate::auth::jwt::{
    verify_jwt
};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: String,
}

pub async fn auth(mut req: Request<Body>, next: Next) -> Result<Response, String> {

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    if let Some(token) = auth_header {

        if token.starts_with("Bearer ") {
            let token = token.trim_start_matches("Bearer ");

            if let Some(claims) = verify_jwt(&token, &env::var("JWT_SECRET").unwrap()) {
                req.extensions_mut().insert(AuthUser {
                    user_id: claims.sub
                });
                return Ok(next.run(req).await);
            }
        }
    }

    Err("Unauthorized".into())
}