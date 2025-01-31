use std::{
    env,
    fs::{self, File},
    path::Path,
};

use actix_web::http::header::ContentDisposition;
use uuid::Uuid;

use crate::models::error::{AppError, AppErrorType};

pub fn file_handle(content_disposition: &ContentDisposition) -> Result<(File, String), AppError> {
    let upload_dir = env::var("UPLOAD_DIR").unwrap();
    fs::create_dir_all(&upload_dir).map_err(|_| AppError {
        error_type: AppErrorType::FailedToUpload,
        message: Some(String::from("Failed to upload file please try again later")),
    })?;
    let u_file_name = content_disposition.get_filename().ok_or_else(|| AppError {
        error_type: AppErrorType::FileError,
        message: Some(String::from("failed to get filename")),
    })?;
    let sanitize_name = sanitize_filename::sanitize(u_file_name);
    let file_name = format!("{}-{}", Uuid::new_v4(), sanitize_name);
    let path = Path::new(&upload_dir).join(&file_name);
    let file = fs::File::create(&path).map_err(|_| AppError {
        error_type: AppErrorType::FailedToUpload,
        message: Some(String::from("Failed to create file")),
    })?;

    return Ok((file, file_name));
}
