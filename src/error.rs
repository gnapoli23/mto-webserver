use sea_orm::DbErr;


#[derive(Debug)]
pub enum ServerError {
    ServiceError,
    ApiError,
    DbError(DbErr)
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServerError::ServiceError => todo!(),
            ServerError::ApiError => todo!(),
            ServerError::DbError(e) => write!(f, "Database error: {e}"),
        }
    }
}

impl std::error::Error for ServerError{}