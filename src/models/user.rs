use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserSignUp {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile_url: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub email: String,
    pub profile_url: String,
    pub created_at: chrono::DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
