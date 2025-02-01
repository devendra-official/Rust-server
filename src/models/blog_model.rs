use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Blog {
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct BlogData {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub image_url: String,
    pub created_at: chrono::DateTime<Utc>,
    pub email: String,
    pub username: String,
    pub profile_image: String,
}
