use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: String, secret: &str) -> String {

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    )
    .unwrap()
}

pub fn verify_jwt(token: &str, secret: &str) -> Option<Claims> {

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).unwrap();

    println!("User id: {}", decoded.claims.sub);
    Some(decoded.claims)
}