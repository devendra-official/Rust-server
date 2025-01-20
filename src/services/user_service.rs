use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::controller::jwt::generate_jwt;
use crate::controller::pass_hash::{CusHashing, CusPasswordHash};
use crate::models::error_res::{CusResponse, ErrorType, ServerError};
use crate::models::user::{User, UserLogin, UserSignUp};

pub async fn login(
    user: web::Json<UserLogin>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, ServerError> {
    let user_data = user.into_inner();
    let pool: Pool<Postgres> = pool.get_ref().clone();
    let sql = "SELECT * FROM users WHERE email=$1";

    match sqlx::query(sql)
        .bind(user_data.email)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => {
            let id: Uuid = row.get("id");
            let username: String = row.get("username");
            let name: String = row.get("name");
            let email: String = row.get("email");
            let password: String = row.get("password");
            let profile_url: String = row.get("profile_url");
            let created_at: chrono::DateTime<Utc> = row.get("created_at");
            let token = generate_jwt(id.to_string()).unwrap();

            let is_correct = match CusPasswordHash::password_verify(&password, &user_data.password)
            {
                Ok(valid) => valid,
                Err(error) => {
                    return Err(ServerError {
                        message: error,
                        error_type: ErrorType::ControllerError,
                    })
                }
            };

            if is_correct {
                let response = User {
                    username,
                    name,
                    email,
                    profile_url,
                    created_at,
                    token: Some(token),
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                return Err(ServerError {
                    message: "wrong password".to_string(),
                    error_type: ErrorType::WrongPassword,
                });
            }
        }
        Err(sqlx::Error::RowNotFound) => Err(ServerError {
            message: "user not found".to_string(),
            error_type: ErrorType::NotFound,
        }),
        Err(error) => Err(ServerError {
            message: error.to_string(),
            error_type: ErrorType::DBError,
        }),
    }
}

pub async fn signup(
    user: web::Json<UserSignUp>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, ServerError> {
    let usr = user.into_inner();
    let pool: Pool<Postgres> = pool.get_ref().clone();

    let hash = match CusPasswordHash::password_hash(&usr.password) {
        Ok(hash) => hash,
        Err(error) => return Err(ServerError {
            message: error,
            error_type: ErrorType::ControllerError,
        }),
    };

    let query =
        "INSERT INTO users (name,username,email,password,profile_url) VALUES ($1,$2,$3,$4,$5)";

    let _result = match sqlx::query(query)
        .bind(usr.name)
        .bind(usr.username)
        .bind(usr.email)
        .bind(hash)
        .bind(usr.profile_url)
        .execute(&pool)
        .await
    {
        Ok(_result) => {
           return Ok(HttpResponse::Created().json(CusResponse::new("user successfully created")));
        },
        Err(_error) => {
            return Err(ServerError { message: "something went wrong!".to_string(), error_type: ErrorType::UserCreateError });
        }
    };
}
