use serde::{Serialize, Deserialize};
use validator::{Validate};
use rocket_okapi::JsonSchema;

#[derive(Serialize, Deserialize, Validate, JsonSchema)]
pub struct UserValidateDTO {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}