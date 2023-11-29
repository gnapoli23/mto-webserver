use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use log::info;
use mto_service::crud;
use sea_orm::DatabaseConnection;

#[post("/")]
pub async fn add_request(
    db: web::Data<DatabaseConnection>,
    data: web::Json<serde_json::Value>,
) -> Result<HttpResponse, Error> {
    let data = crud::request::add_request(&db).await.unwrap();
    Ok(HttpResponse::Ok().json(&data))
}

#[get("/{id}")]
pub async fn get_request(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let data = crud::request::get_request(&db, *id).await.unwrap();
    Ok(HttpResponse::Ok().json(&data))
}

#[put("/{id}")]
pub async fn update_request(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let data = crud::request::update_request(&db).await.unwrap();
    Ok(HttpResponse::Ok().json(&data))
}

#[delete("/{id}")]
pub async fn delete_request(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let data = crud::request::delete_request(&db).await.unwrap();
    Ok(HttpResponse::Ok().json(&data))
}
