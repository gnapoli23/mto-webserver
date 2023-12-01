use sea_orm::DbErr;

#[derive(Debug)]
pub enum ServiceError {
    Crud(DbErr),
    DataNotFound,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServiceError::Crud(e) => write!(f, "CRUD service serror: {e}"),
            Self::DataNotFound => write!(f, "Data not found"),
        }
    }
}

impl std::error::Error for ServiceError {}
