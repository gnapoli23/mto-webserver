use mto_entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

// TODO: change here with DbError
pub async fn add_request(conn: &DatabaseConnection) -> Result<RequestModel, DbErr> {
    todo!()
}

// TODO: change here with DbError
pub async fn get_request(
    conn: &DatabaseConnection,
    id: i32,
) -> Result<Option<RequestModel>, DbErr> {
    Request::find_by_id(id).one(conn).await
}

// TODO: change here with DbError
pub async fn update_request(conn: &DatabaseConnection) -> Result<RequestModel, DbErr> {
    todo!()
}

// TODO: change here with DbError
pub async fn delete_request(conn: &DatabaseConnection) -> Result<RequestModel, DbErr> {
    todo!()
}
