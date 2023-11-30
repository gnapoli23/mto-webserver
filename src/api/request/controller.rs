use actix_web::{delete, get, post, put, web, HttpResponse, Error};
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
    let data = service::add_request(&db, data.0)
        .await?;
    let resp = ApiResponse::new(data);
    Ok(HttpResponse::Ok().json(&resp))
}

#[get("/{id}")]
pub async fn get_request(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let data = service::get_request(&db, *id)
        .await?;
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
