use mto_entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait};

pub async fn add_request(conn: &DatabaseConnection) -> Result<RequestModel, DbErr> {
    todo!()
}

pub async fn get_request(
    conn: &DatabaseConnection,
    id: i32,
) -> Result<Option<RequestModel>, DbErr> {
    Request::find_by_id(id).one(conn).await
}

pub async fn update_request(conn: &DatabaseConnection) -> Result<RequestModel, DbErr> {
    todo!()
}

pub async fn delete_request(conn: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    let request: RequestActiveModel = Request::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::Custom(format!(
            "Request with id {id} does not exist."
        )))
        .map(Into::into)?;
    request.delete(conn).await
}
