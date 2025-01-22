use crate::models::other_model::Claims;
use jsonwebtoken::{EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Jwt {
    fn generate_jwt(id: String) -> Result<String, String>;
}

pub struct Jwtoken;

impl Jwt for Jwtoken {
    fn generate_jwt(id: String) -> Result<String, String> {
        let expiration_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("failed to create expire")
            .as_secs()
            + 3600;

        let claims = Claims {
            company: String::from("user_authentication"),
            exp: expiration_time as usize,
            sub: id,
        };

        let key = match std::env::var("JWT_KEY") {
            Ok(value) => value,
            Err(err) => return Err(err.to_string()),
        };

        let headers = Header::default();

        match jsonwebtoken::encode(&headers, &claims, &EncodingKey::from_secret(key.as_ref())) {
            Ok(token) => Ok(token),
            Err(error) => Err(error.to_string()),
        }
    }
}
