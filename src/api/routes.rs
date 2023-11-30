use std::error::Error;

use crate::{
    api::{request::{self, HttpBinPayload}, ApiResponse},
    utils::find_mto_numbers,
};
use actix_web::{get, web, HttpResponse};
use futures_util::{stream::FuturesUnordered, StreamExt};
use rand::{distributions::Uniform, Rng};
use sea_orm::DatabaseConnection;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/request").configure(request::config))
        .service(run);
}

#[get("/run")]
async fn run(conn: web::Data<DatabaseConnection>) -> Result<HttpResponse, Box<dyn Error>> {
    // Send 30 POST requests to https://httpbin.org/post
    // Generate random number in range [0,10]
    let range = Uniform::<u8>::from(0..11);
    let value_requests = rand::thread_rng()
        .sample_iter(&range)
        .take(30)
        .map(HttpBinPayload::new)
        .collect::<Vec<HttpBinPayload>>();

    // Create a future for each value, to run the requests concurrently
    let client = reqwest::Client::new();
    let futs = value_requests
        .iter()
        .map(|req| request::send_request(&client, req, &conn))
        .collect::<FuturesUnordered<_>>();

    let res: Vec<u8> = futs
        .collect::<Vec<Option<_>>>()
        .await
        .into_iter()
        .flatten()
        .collect();

    // Find values that appear more than once
    let mto = find_mto_numbers(res);
    let res = ApiResponse::new(Some(mto));

    Ok(HttpResponse::Ok().json(res))
}
