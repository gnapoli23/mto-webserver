use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestDto {
    pub id: i32,
    pub value: i32,
}
