use serde::Serialize;

#[derive(Serialize)]
pub struct Claims {
    pub company: String,
    pub sub: String,
    pub exp: usize,
}