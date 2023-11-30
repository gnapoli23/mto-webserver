use mto_entity::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DeleteResult, EntityTrait};

use crate::error::ServerError;

use super::dto::RequestDto;

pub async fn add_request(
    conn: &DatabaseConnection,
    data: RequestDto,
) -> Result<RequestModel, ServerError> {
    // Add request
    let new_request = RequestActiveModel {
        id: Set(data.id),
        batch_id: Set(None),
        value: Set(data.value),
        status: Set(None),
    };
    new_request.insert(conn).await.map_err(ServerError::DbError)
}

pub async fn get_request(conn: &DatabaseConnection, id: i32) -> Result<RequestModel, ServerError> {
    // Find request
    Request::find_by_id(id)
        .one(conn)
        .await
        .map_err(ServerError::DbError)?
        .ok_or(ServerError::NotFound)
}

pub async fn update_request(
    conn: &DatabaseConnection,
    data: RequestDto,
) -> Result<RequestModel, ServerError> {
    let mut request: RequestActiveModel = Request::find_by_id(data.id)
        .one(conn)
        .await
        .map_err(ServerError::DbError)?
        .ok_or(ServerError::NotFound)
        .map(Into::into)?;
    request.id = Set(data.id);
    request.value = Set(data.value);

    let data = request.update(conn).await?;

    Ok(data)
}

pub async fn delete_request(
    conn: &DatabaseConnection,
    id: i32,
) -> Result<DeleteResult, ServerError> {
    let request: RequestActiveModel = Request::find_by_id(id)
        .one(conn)
        .await
        .map_err(ServerError::DbError)?
        .ok_or(ServerError::NotFound)
        .map(Into::into)?;
    request.delete(conn).await.map_err(ServerError::DbError)
}
