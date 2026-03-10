use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordHash, PasswordVerifier},
    Argon2
};
use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> String {

    // random salt yaratish
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 instance
    let argon2 = Argon2::default();

    // password hash qilish
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hash
}

pub fn verify_password(hash: &str, password: &str) -> bool {

    let parsed_hash = PasswordHash::new(hash).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}