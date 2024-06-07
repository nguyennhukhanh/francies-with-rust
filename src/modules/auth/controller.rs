use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::post;
use rocket::response::status;
use rocket_okapi::{openapi};
use validator::Validate;

use super::service::AuthService;
use super::dto::user_validate::UserValidateDTO;
use crate::shared::structs::response::{Meta, ErrorResponse};

#[openapi(tag = "auth")]
#[post("/auth/login", data = "<login_info>")]
pub fn login(login_info: Json<UserValidateDTO>) -> Result<status::Custom<&'static str>, status::Custom<Json<ErrorResponse>>> {
    if let Err(e) = login_info.validate() {
       
        let error_response = ErrorResponse {
            meta: Meta {
                code: Status::BadRequest.code,
                message: e.to_string(),
            },
            data: serde_json::Value::Object(Default::default()),
        };
        return Err(status::Custom(Status::BadRequest, Json(error_response)));
    }

    let auth_service = AuthService::new();
    if auth_service.authenticate(&login_info.email, &login_info.password) {
        Ok(status::Custom(Status::Ok, "Logged in successfully."))
    } else {
        let error_response = ErrorResponse {
            meta: Meta {
                code: Status::Unauthorized.code,
                message: "Invalid email or password.".to_string(),
            },
            data: serde_json::Value::Object(Default::default()),
        };
        Err(status::Custom(Status::Unauthorized, Json(error_response)))
    }
}

#[openapi(tag = "auth")]
#[get("/auth/test")]
pub fn test_auth() -> status::Custom<&'static str> {
    status::Custom(Status::Ok, "Tested successfully.")
}

pub fn routes() -> Vec<rocket::Route> {
    rocket_okapi::openapi_get_routes![login, test_auth]
}