use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Blog {
    pub title: String,
    pub content: String,
}