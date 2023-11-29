pub mod api;
pub mod config;
pub mod db;
pub mod utils;

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

pub async fn main() -> std::io::Result<()> {
    // Init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init DB
    let config = config::get().await;
    let conn = db::connect(&config.db).await.unwrap(); // TODO: handle error
    let state = web::Data::new(conn);

    // Init server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(api::routes::config)
    })
    .disable_signals()
    .bind(("localhost", 8080))?;
    info!("Starting server");
    server.run().await
}
