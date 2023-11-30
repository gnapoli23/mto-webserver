mod controller;
mod dto;
mod service;
pub use service::send_request;

use actix_web::web;
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::add_request)
        .service(controller::get_request)
        .service(controller::update_request)
        .service(controller::delete_request);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpBinPayload {
    pub value: u8,
}

impl HttpBinPayload {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpBinResponse {
    data: String,
}
