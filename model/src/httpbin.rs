use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpBinRequest {
    pub value: u8,
}

impl HttpBinRequest {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpBinResponse {
    pub data: String,
}
