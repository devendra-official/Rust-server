use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::utils::jwt::{Jwt, Jwtoken};
use crate::utils::pass_hash::{CusHashing, CusPasswordHash};
use crate::models::error::{AppError, AppErrorType, AppRes};
use crate::models::user::{User, UserLogin, UserSignUp};

pub async fn login(
    user: web::Json<UserLogin>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, AppError> {
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
            let token = match Jwtoken::generate_jwt(id.to_string()) {
                Ok(value) => value,
                Err(_) => {
                    return Err(AppError {
                        message: Some("Invalid credentials, failed to generate JWT.".to_string()),
                        error_type: AppErrorType::ControllerError,
                    });
                }
            };

            let is_correct = match CusPasswordHash::password_verify(&password, &user_data.password)
            {
                Ok(valid) => valid,
                Err(_) => {
                    return Err(AppError {
                        message: None,
                        error_type: AppErrorType::ControllerError,
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
                return Err(AppError {
                    message: Some("wrong password".to_string()),
                    error_type: AppErrorType::WrongPassword,
                });
            }
        }
        Err(sqlx::Error::RowNotFound) => Err(AppError {
            message: Some("user not found".to_string()),
            error_type: AppErrorType::NotFound,
        }),
        Err(_) => Err(AppError {
            message: None,
            error_type: AppErrorType::DBError,
        }),
    }
}

pub async fn signup(
    user: web::Json<UserSignUp>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, AppError> {
    let usr = user.into_inner();
    let pool: Pool<Postgres> = pool.get_ref().clone();

    let hash = match CusPasswordHash::password_hash(&usr.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(AppError {
                message: None,
                error_type: AppErrorType::ControllerError,
            })
        }
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
            return Ok(HttpResponse::Created().json(AppRes::new("user successfully created")));
        }
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            return Err(AppError {
                message: Some("user with same data already exist".to_string()),
                error_type: AppErrorType::ControllerError,
            })
        }
        Err(_) => {
            return Err(AppError {
                message: Some("something went wrong!!!".to_string()),
                error_type: AppErrorType::Other,
            })
        }
    };
}
