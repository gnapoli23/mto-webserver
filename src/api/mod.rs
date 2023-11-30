use serde::Serialize;

pub mod request;
pub mod routes;

#[derive(Serialize, Debug, Clone)]
pub struct ApiResponse<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data: Some(data) }
    }
}
