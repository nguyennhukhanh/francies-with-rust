use rocket::serde::{Serialize, Deserialize};
use schemars::JsonSchema; 

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Meta {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ErrorResponse {
    pub meta: Meta,
    pub data: serde_json::Value,
}