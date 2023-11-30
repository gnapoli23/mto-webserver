use actix_web::{HttpResponse, ResponseError};
use log::error;
use sea_orm::DbErr;

#[derive(Debug)]
pub enum ServerError {
    NotFound,
    WrongCredentials,
    DbError(DbErr),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServerError::NotFound => write!(f, "Data not found"),
            ServerError::WrongCredentials => write!(f, "Username and password are incorrect"),
            ServerError::DbError(e) => write!(f, "Database error: {e}"),
        }
    }
}

impl std::error::Error for ServerError {}

impl ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match &self {
            ServerError::NotFound => {
                error!("Data not found!");
                HttpResponse::NotFound().finish()
            }
            Self::WrongCredentials => {
                error!("Username and password are incorrect!");
                HttpResponse::Unauthorized().finish()
            }
            ServerError::DbError(e) => {
                error!("Database error: {e}");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

impl From<DbErr> for ServerError {
    fn from(value: DbErr) -> Self {
        ServerError::DbError(value)
    }
}
