pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod utils;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::info;

pub async fn main() -> std::io::Result<()> {
    // Load .env file and init configuration
    dotenv().ok();
    let config = config::get().await.unwrap();

    // Init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init DB
    let conn = db::connect(config).await.unwrap(); // TODO: handle error
    let state = web::Data::new(conn);

    // Init server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(api::routes::config)
    })
    .disable_signals()
    .bind((config.server_host.as_ref(), config.server_port))?;
    info!("Starting server");
    server.run().await
}
