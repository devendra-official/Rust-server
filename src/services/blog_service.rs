use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web::Data, HttpMessage, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    models::{
        blog_model::Blog,
        error::{AppError, AppErrorType, AppRes},
    },
    utils::files::file_handle,
};
const MAX_SIZE: usize = 10_000_000;

pub async fn blog_upload(
    mut payload: Multipart,
    pool: Data<Pool<Postgres>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut image_url: Option<String> = None;
    let mut blog_data: Option<Blog> = None;
    let pool: Pool<Postgres> = pool.get_ref().clone();
    let ext = req.extensions();
    let id = match ext.get::<String>() {
        Some(id) => id,
        None => {
            return Err(AppError {
                error_type: AppErrorType::NotAllowed,
                message: Some("Invalid token".to_string()),
            })
        }
    };
    let user_id = Uuid::parse_str(id).unwrap();

    while let Some(mut field) = payload.try_next().await.map_err(|error| AppError {
        error_type: AppErrorType::FileError,
        message: Some(error.to_string()),
    })? {
        let content_disposition = field.content_disposition().ok_or_else(|| AppError {
            message: Some(String::from("Meta Data not found")),
            error_type: AppErrorType::FileError,
        })?;

        if let Some(name) = content_disposition.get_name() {
            match name {
                "blog" => {
                    let mut data = String::new();
                    while let Some(chunk) = field.try_next().await.map_err(|_| AppError {
                        error_type: AppErrorType::FileError,
                        message: Some(String::from("Failed to get data")),
                    })? {
                        data.push_str(&String::from_utf8_lossy(&chunk));
                    }
                    let blog: Blog = serde_json::from_str(&data).map_err(|_| AppError {
                        error_type: AppErrorType::InsuffiecientField,
                        message: Some(String::from("Insufficient field")),
                    })?;
                    blog_data = Some(blog);
                }
                "file" => {
                    let (mut file, filename) =
                        file_handle(&content_disposition).map_err(|error| return error)?;
                    image_url = Some(filename);
                    let mut total_size = 0;
                    while let Some(chunk) = field.try_next().await.map_err(|_| AppError {
                        error_type: AppErrorType::FileError,
                        message: Some(String::from("Failed to get files")),
                    })? {
                        total_size += chunk.len();
                        if total_size > MAX_SIZE {
                            return Err(AppError {
                                message: Some("Max File Size".to_string()),
                                error_type: AppErrorType::LargeFile,
                            });
                        }
                        file.write_all(&chunk).map_err(|_| AppError {
                            error_type: AppErrorType::FailedToUpload,
                            message: Some(String::from(
                                "something went wrong please try again later",
                            )),
                        })?;
                    }
                }
                _ => {}
            }
        }
    }
    if let Some(data) = blog_data {
        if let Some(url) = image_url {
            let query =
                "INSERT INTO blog_posts (title,content,author_id,image_url) VALUES($1,$2,$3,$4)";
            sqlx::query(query)
                .bind(data.title)
                .bind(data.content)
                .bind(user_id)
                .bind(url)
                .execute(&pool)
                .await
                .map_err(|error| AppError {
                    error_type: AppErrorType::DBError,
                    message: Some(error.to_string()),
                })?;
        }
    }

    Ok(HttpResponse::Ok().json(AppRes::new("Blog uploaded successfully")))
}
