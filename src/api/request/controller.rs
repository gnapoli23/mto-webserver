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
    use actix_web::{web, App};
    use mto_entity::prelude::*;
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult};

    use crate::api::{
        request::{
            controller::{add_request, delete_request, get_request, update_request},
            dto::RequestDto,
        },
        ApiResponse,
    };

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
    async fn test_request_post() {
        // Setup mock DB
        let db = setup_db();

        // Setup api
        let state = web::Data::new(db);
        let app =
            actix_web::test::init_service(App::new().app_data(state.clone()).service(add_request))
                .await;

        let data = RequestDto {
            id: 123,
            value: 123,
        };
        let req = actix_web::test::TestRequest::post()
            .uri("/")
            .set_json(data)
            .to_request();
        let resp: ApiResponse<RequestModel> =
            actix_web::test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.clone().data.unwrap().id, 123);
        assert_eq!(resp.data.unwrap().value, 123);
    }

    #[tokio::test]
    async fn test_request_get() {
        // Setup mock DB
        let db = setup_db();

        // Setup api
        let state = web::Data::new(db);
        let app =
            actix_web::test::init_service(App::new().app_data(state.clone()).service(get_request))
                .await;

        let req = actix_web::test::TestRequest::get().uri("/123").to_request();
        let resp: ApiResponse<RequestModel> =
            actix_web::test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.clone().data.unwrap().id, 123);
        assert_eq!(resp.data.unwrap().value, 123);
    }

    #[tokio::test]
    async fn test_request_update() {
        // Setup mock DB
        let db = setup_db();

        // Setup api
        let state = web::Data::new(db);
        let app = actix_web::test::init_service(
            App::new().app_data(state.clone()).service(update_request),
        )
        .await;

        let data = RequestDto {
            id: 321,
            value: 111,
        };
        let req = actix_web::test::TestRequest::put()
            .uri("/321")
            .set_json(data)
            .to_request();

        let resp: ApiResponse<RequestModel> =
            actix_web::test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.clone().data.unwrap().id, 321);
        assert_eq!(resp.data.unwrap().value, 111);
    }

    #[tokio::test]
    async fn test_request_delete() {
        // Setup mock DB
        let db = setup_db();

        // Setup api
        let state = web::Data::new(db);
        let app = actix_web::test::init_service(
            App::new().app_data(state.clone()).service(delete_request),
        )
        .await;

        let req = actix_web::test::TestRequest::delete()
            .uri("/123")
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
