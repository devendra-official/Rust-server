use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Claims {
    pub company: String,
    pub sub: String,
    pub exp: usize,
}
