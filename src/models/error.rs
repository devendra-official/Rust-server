use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug)]
pub enum AppErrorType {
    DBError,
    ControllerError,
    WrongPassword,
    NotFound,
    Other,
    FileError,
    FailedToUpload,
    LargeFile,
    InsuffiecientField,
    Authentication,
    NotAllowed
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppRes {
    pub message: String,
}

impl AppRes {
    pub fn new(message:&str) -> Self {
        AppRes { message: message.to_string() }
    }
}

impl AppError {
    fn message(&self) -> String {
        match &self.message {
            Some(message) => message.clone(),
            None => String::from("an unexpected error occured")
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::ControllerError => StatusCode::BAD_REQUEST,
            AppErrorType::DBError => StatusCode::SERVICE_UNAVAILABLE,
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
            AppErrorType::Other => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::WrongPassword => StatusCode::UNAUTHORIZED,
            AppErrorType::FailedToUpload => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::FileError => StatusCode::BAD_REQUEST,
            AppErrorType::LargeFile => StatusCode::PAYLOAD_TOO_LARGE,
            AppErrorType::InsuffiecientField => StatusCode::BAD_REQUEST,
            AppErrorType::Authentication => StatusCode::UNAUTHORIZED,
            AppErrorType::NotAllowed => StatusCode::FORBIDDEN
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppRes {
            message: self.message(),
        })
    }
}
