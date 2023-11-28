pub mod api;
pub mod config;
pub mod db;
pub mod service;
use std::{collections::HashMap, env, error::Error, f32::consts::E, process::Output};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use futures_util::{stream::FuturesUnordered, Future, FutureExt, StreamExt};
use log::{error, info};
use rand::{distributions::Uniform, Rng};
use reqwest::{Body, Client};
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

impl HttpBinRequest {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HttpBinResponse {
    data: String,
}

#[get("/run")]
async fn run(data: web::Data<DatabaseConnection>) -> Result<HttpResponse, Box<dyn Error>> {
    // Send 30 POST requests to https://httpbin.org/post
    // Generate random number in range [0,10]
    let range = Uniform::<u8>::from(0..11);
    let value_requests: Vec<HttpBinRequest> = rand::thread_rng()
        .sample_iter(&range)
        .take(30)
        .map(HttpBinRequest::new)
        .collect();
    //info!("Generated random values: {value_requests:?}");
    //let futs = value_requests.into_iter()

    // Genero le future delle richieste con un iteratore
    let client = reqwest::Client::new();
    let futs = value_requests
        .iter()
        .map(|req| send(&client, req))
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

async fn send(client: &Client, req: &HttpBinRequest) -> Option<u8> {
    let fut = client.post("https://httpbin.org/post").json(req).send();
    match fut.await {
        Ok(res) => match res.json::<HttpBinResponse>().await {
            Ok(json_res) => {
                if let Ok(data) = serde_json::from_str::<HttpBinRequest>(&json_res.data) {
                    Some(data.value)
                } else {
                    error!("Unable to deserialize response");
                    None
                }
            }
            Err(e) => {
                error!("Unable to deserialize response into JSON: {e}");
                None
            }
        },
        Err(e) => {
            error!("Unable to send request: {e}");
            None
        }
    }
}

fn find_mto_numbers(values: Vec<u8>) -> Vec<u8> {
    let mut count_map = HashMap::new();

    // Count occurrences of each number
    values
        .iter()
        .for_each(|v| *count_map.entry(v).or_insert(0) += 1);

    // Find numbers that appear more than once
    let mut mto_values = count_map
        .into_iter()
        .filter_map(|(&num, count)| if count > 1 { Some(num) } else { None })
        .collect::<Vec<u8>>();

    // Sort the result in ascending order
    mto_values.sort();

    mto_values
}
