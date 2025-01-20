use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorType {
    DBError,
    ControllerError,
    UserCreateError,
    WrongPassword,
    NotFound,
}

#[derive(Debug)]
pub struct ServerError {
    pub message: String,
    pub error_type: ErrorType,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Serialize)]
pub struct CusResponse {
    message: String,
}

impl CusResponse {
    pub fn new(msg: &str) -> Self {
        CusResponse {
            message: msg.to_string(),
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(CusResponse::new(&self.message))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_type {
            ErrorType::ControllerError => StatusCode::INTERNAL_SERVER_ERROR, 
            ErrorType::DBError => StatusCode::SERVICE_UNAVAILABLE, 
            ErrorType::WrongPassword => StatusCode::UNAUTHORIZED,  
            ErrorType::UserCreateError => StatusCode::BAD_REQUEST, 
            ErrorType::NotFound => StatusCode::NOT_FOUND
        }
    }
}
