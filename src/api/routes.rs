use mto_model::httpbin::HttpBinRequest;
use mto_service::send_httpbin_request;
use std::{collections::HashMap, error::Error};

use crate::{
    api::{
        request::{self},
        ApiResponse,
    },
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
        .map(HttpBinRequest::new)
        .collect::<Vec<HttpBinRequest>>();

    // Create a future for each value, to run the requests concurrently
    let client = reqwest::Client::new();
    let futs = value_requests
        .iter()
        .map(|req| send_httpbin_request(&client, req, &conn))
        .collect::<FuturesUnordered<_>>();

    let res: Vec<u8> = futs
        .collect::<Vec<Option<_>>>()
        .await
        .into_iter()
        .flatten()
        .collect();

    // Find values that appear more than once
    let mto = find_mto_numbers(res);
    let mut res_data: HashMap<String, Vec<u8>> = HashMap::new();
    res_data.insert(
        "sent_values".into(),
        value_requests.into_iter().map(|v| v.value).collect(),
    );
    res_data.insert("mto_values".into(), mto);
    let res = ApiResponse::new(Some(res_data));

    Ok(HttpResponse::Ok().json(res))
}
