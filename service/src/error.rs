use actix_web::{HttpResponse, ResponseError};
use log::error;
use sea_orm::DbErr;

#[derive(Debug)]
pub enum ServiceError {
    Crud(DbErr),
    NotFound,
    Unauthorized,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServiceError::Crud(e) => write!(f, "CRUD service serror: {e}"),
            ServiceError::NotFound => write!(f, "Data not found"),
            ServiceError::Unauthorized => write!(f, "Username and password are incorrect"),
        }
    }
}

impl std::error::Error for ServiceError {}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::Crud(e) => {
                error!("Database error: {e}");
                HttpResponse::InternalServerError().finish()
            }
            ServiceError::NotFound => {
                error!("Data not found!");
                HttpResponse::NotFound().finish()
            }
            ServiceError::Unauthorized => {
                error!("Username and password are incorrect");
                HttpResponse::Unauthorized().finish()
            }
        }
    }
}
