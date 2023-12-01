use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use mto_model::entity::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::error::ServiceError;

pub async fn basic_auth(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if let Some(db) = req.app_data::<web::Data<DatabaseConnection>>() {
        // Find user
        let user = User::find()
            .filter(UserColumn::Username.eq(credentials.user_id()))
            .one(db.as_ref())
            .await
            .unwrap();

        // Check credentials
        if let Some(user) = user {
            if user.password.as_deref() == credentials.password() {
                Ok(req)
            } else {
                Err((ServiceError::Unauthorized.into(), req))
            }
        } else {
            Err((ServiceError::NotFound.into(), req))
        }
    } else {
        Err((
            ServiceError::Crud(sea_orm::DbErr::Custom("Unable to get db connection".into())).into(),
            req,
        ))
    }
}
