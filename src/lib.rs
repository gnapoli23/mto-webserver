pub mod config;
pub mod db;
use std::error::Error;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use env_logger::Env;
use log::info;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn run() -> std::io::Result<()> {
    // Init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init DB
    let config = config::get().await;
    let conn = db::connect(&config.db).await.unwrap();
    let state = web::Data::new(conn);

    // Init server
    let server = HttpServer::new(move || App::new().app_data(state.clone()).service(greet).service(run_httpbin))
        .disable_signals()
        .bind(("localhost", 8080))?;
    info!("Starting server");
    server.run().await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/run")]
async fn run_httpbin(data: web::Data<DatabaseConnection>) -> Result<HttpResponse, Box<dyn Error>> {
    let res = db::User::find().all(data.get_ref()).await?;
    Ok(HttpResponse::Ok().json(res))
}
