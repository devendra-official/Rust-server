use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub trait CusHashing {
    fn password_hash(password: &str) -> Result<String, String>;
    fn password_verify(hash: &str,password: &str) -> Result<bool, String>;
}
pub struct CusPasswordHash;

impl CusHashing for CusPasswordHash {
    fn password_hash(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => return Ok(hash.to_string()),
            Err(_) => return Err("failed to create hash".to_string()),
        }
    }
    fn password_verify(hash: &str,password: &str) -> Result<bool, String> {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(hash) => hash,
            Err(_error) => return Err("can't verify password".to_string()),
        };
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}