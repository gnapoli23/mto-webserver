use log::error;
use mto_model::{
    entity::prelude::*,
    httpbin::{HttpBinRequest, HttpBinResponse},
};
use reqwest::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub mod auth;
pub mod crud;
pub mod error;

pub async fn send_httpbin_request(
    client: &Client,
    req: &HttpBinRequest,
    conn: &DatabaseConnection,
) -> Option<u8> {
    let fut = client.post("https://httpbin.org/post").json(req).send();
    let resp = match fut.await {
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
    };

    // Save request on DB
    let request = RequestActiveModel {
        value: Set(req.value.into()),
        ..Default::default()
    };
    if let Err(e) = request.insert(conn).await {
        error!("Unable to save data into database: {e}");
    }

    resp
}
