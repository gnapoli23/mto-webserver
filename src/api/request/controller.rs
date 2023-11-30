use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::api::{
    request::{dto::RequestDto, service},
    ApiResponse,
};

#[post("/")]
pub async fn add_request(
    db: web::Data<DatabaseConnection>,
    data: web::Json<RequestDto>,
) -> Result<HttpResponse, Error> {
    let data = service::add_request(&db, data.0).await?;
    let resp = ApiResponse::new(data);
    Ok(HttpResponse::Ok().json(&resp))
}

#[get("/{id}")]
pub async fn get_request(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let data = service::get_request(&db, *id).await?;
    let resp = ApiResponse::new(Some(data));
    Ok(HttpResponse::Ok().json(&resp))
}

#[put("/{id}")]
pub async fn update_request(
    db: web::Data<DatabaseConnection>,
    data: web::Json<RequestDto>,
) -> Result<HttpResponse, Error> {
    let data = service::update_request(&db, data.0).await?;
    let resp = ApiResponse::new(data);
    Ok(HttpResponse::Ok().json(&resp))
}

#[delete("/{id}")]
pub async fn delete_request(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let _ = service::delete_request(&db, *id).await?;
    Ok(HttpResponse::Ok().finish())
}


#[cfg(test)]
mod controller_tests {
    use sea_orm::{DatabaseConnection, MockExecResult, DatabaseBackend, MockDatabase};
    use mto_entity::prelude::*;

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

    fn setup_app() {

    }


    #[tokio::test]
    async fn test_request_post() {
        //let app = actix_web::test::init_service(app)
    }
}