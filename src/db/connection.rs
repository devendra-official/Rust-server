use log::{info,error};
use sqlx::{Pool, Postgres};

pub async fn connect_db() -> Result<Pool<Postgres>,String> {
    let url = std::env::var("DATABASE_URL").unwrap();
    match Pool::connect(&url).await {
        Ok(pool) => {
            info!("connected to database");
            Ok(pool)
        },
        Err(error) => {
            error!("{}",error.to_string());
            return Err(error.to_string())
        }
    }
}