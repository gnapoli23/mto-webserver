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
        value: Set(data.value),
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

#[cfg(test)]
mod service_tests {
    use sea_orm::MockExecResult;
    use super::*;

    use sea_orm::{DatabaseBackend, MockDatabase};

    use crate::api::request::dto::RequestDto;

    fn setup_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results([
                [RequestModel {
                    id: 123,
                    value: 123,
                }],
                [RequestModel {
                    id: 321,
                    value: 111,
                }],
            ])
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 123,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 321,
                    rows_affected: 1,
                },
            ])
            .into_connection()
    }


    #[tokio::test]
    pub async fn test_add_request() -> Result<(), ServerError> {
        // Create MockDatabase
        let db = setup_db();

        // Call service
        let request_dto = RequestDto { id: 123, value: 123 };
        let resp = add_request(&db, request_dto).await?;
        
        assert_eq!(
            resp,
            RequestModel {
                id: 123,
                value: 123,
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_request() -> Result<(), ServerError> {
        // Create MockDatabase
        let db = setup_db();

        // Call service
        let resp = get_request(&db, 123).await?;
        
        assert_eq!(
            resp,
            RequestModel {
                id: 123,
                value: 123,
            }
        );

        Ok(())
    }


    #[tokio::test]
    async fn test_update_request() -> Result<(), ServerError> {
        // Create MockDatabase
        let db = setup_db();

        // Call service
        let request_dto = RequestDto { id: 321, value: 111 };
        let resp = update_request(&db, request_dto).await?;
        
        assert_eq!(
            resp,
            RequestModel {
                id: 321,
                value: 111,
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_request() -> Result<(), ServerError> {
        // Create MockDatabase
        let db = setup_db();

        // Call service
        let resp = delete_request(&db, 321).await?;
        
        assert_eq!(
            resp.rows_affected,
            1
        );

        Ok(())
    }
}
