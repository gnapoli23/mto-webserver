use actix_web::{delete, get, post, put, Error, HttpResponse};

#[post("/")]
pub async fn add_request() -> Result<HttpResponse, Error> {
    todo!()
}

#[get("/")]
pub async fn get_request() -> Result<HttpResponse, Error> {
    todo!()
}

#[put("/")]
pub async fn update_request() -> Result<HttpResponse, Error> {
    todo!()
}

#[delete("/")]
pub async fn delete_request() -> Result<HttpResponse, Error> {
    todo!()
}
