use actix_web::{HttpResponse, ResponseError};
use log::error;
use mto_service::error::ServiceError;

#[derive(Debug)]
pub enum ServerError {
    ServiceError(ServiceError),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServerError::ServiceError(e) => write!(f, "Service Error -> {e}"),
        }
    }
}

impl std::error::Error for ServerError {}

impl ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match &self {
            // Self::WrongCredentials => {
            //     error!("Username and password are incorrect!");
            //     HttpResponse::Unauthorized().finish()
            // }
            ServerError::ServiceError(e) => match e {
                ServiceError::Crud(e) => {
                    error!("Database error: {e}");
                    HttpResponse::InternalServerError().finish()
                }
                ServiceError::DataNotFound => {
                    error!("Data not found!");
                    HttpResponse::NotFound().finish()
                }
            },
        }
    }
}

impl From<ServiceError> for ServerError {
    fn from(value: ServiceError) -> Self {
        ServerError::ServiceError(value)
    }
}
