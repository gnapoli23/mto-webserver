pub mod api;
pub mod config;
pub mod db;
use std::{collections::HashMap, error::Error};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;
use rand::Rng;
use reqwest::Body;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

pub async fn main() -> std::io::Result<()> {
    // Init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init DB
    let config = config::get().await;
    let conn = db::connect(&config.db).await.unwrap();
    let state = web::Data::new(conn);

    // Init server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api").configure(api::httpbin_request::service_config))
            .service(run)
    })
    .disable_signals()
    .bind(("localhost", 8080))?;
    info!("Starting server");
    server.run().await
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HttpBinRequest {
    pub value: u8,
}

#[get("/run")]
async fn run(data: web::Data<DatabaseConnection>) -> Result<HttpResponse, Box<dyn Error>> {

    // Send 30 POST requests to https://httpbin.org/post
    // Generate random number in range [0,10]
    let mut rng = rand::thread_rng();
    let value = rng.gen_range::<u8, _>(0..11);
    info!("Generated random value: {value}");

    let payload = HttpBinRequest { value };
    let client = reqwest::Client::new();
    let res = client
        .post("https://httpbin.org/post")
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(HttpResponse::Ok().json(res))
}
