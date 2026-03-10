use argon2::{
    Argon2,
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, PasswordVerifier, SaltString
    }
};

pub fn hash_password(password: &str) -> String {
    // Generate a secure random salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash the password using the default Argon2 configuration
    let argon2 = Argon2::default();

    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string(); // The result is a PHC string, ready to store in a database

    hashed_password
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    // Parse the stored hash from the PHC string format
    let parsed_hash = argon2::password_hash::PasswordHash::new(hashed_password)
        .expect("Invalid stored hash");

    // Verify the new password against the stored hash
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
