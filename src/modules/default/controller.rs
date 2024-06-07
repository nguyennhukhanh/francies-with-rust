use rocket::get;
use rocket_okapi::{openapi};

#[openapi(tag = "default")]
#[get("/heath")]
fn index() -> &'static str {
    "The application is up and running!"
}

pub fn routes() -> Vec<rocket::Route> {
    rocket_okapi::openapi_get_routes![index]
}