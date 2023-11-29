use std::error::Error;

use crate::{
    api::request,
    utils::httpbin::{find_mto_numbers, send_request, HttpBinPayload},
};
use actix_web::{get, web, HttpResponse};
use futures_util::{stream::FuturesUnordered, StreamExt};
use log::info;
use rand::{distributions::Uniform, Rng};
use sea_orm::DatabaseConnection;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/request").configure(request::config))
        .service(run);
}

// Add here the /run endpoint
#[get("/run")]
async fn run(_data: web::Data<DatabaseConnection>) -> Result<HttpResponse, Box<dyn Error>> {
    // Send 30 POST requests to https://httpbin.org/post
    // Generate random number in range [0,10]
    let range = Uniform::<u8>::from(0..11);
    let value_requests = rand::thread_rng()
        .sample_iter(&range)
        .take(30)
        .map(HttpBinPayload::new)
        .collect::<Vec<HttpBinPayload>>();
    //info!("Generated random values: {value_requests:?}");
    //let futs = value_requests.into_iter()

    // Genero le future delle richieste con un iteratore
    let client = reqwest::Client::new();
    let futs = value_requests
        .iter()
        .map(|req| send_request(&client, req))
        .collect::<FuturesUnordered<_>>();

    let res: Vec<u8> = futs
        .collect::<Vec<Option<_>>>()
        .await
        .into_iter()
        .flatten()
        .collect();

    info!("Res values: {res:?}");
    let mto = find_mto_numbers(res);
    info!("Mto values: {mto:?}");
    Ok(HttpResponse::Ok().json(mto))
}
