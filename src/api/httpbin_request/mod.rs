mod controller;
mod crud;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/httpbin_request")
            .service(controller::add_request)
            .service(controller::get_request)
            .service(controller::update_request)
            .service(controller::delete_request),
    );
}
