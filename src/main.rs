// #[macro_use] extern crate rocket;

// use rocket::shield::Shield;
// use rocket_cors::CorsOptions;
// use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

// mod modules;

// use modules::auth;
// use modules::default;

// #[launch]
// fn rocket() -> _ {
//     let cors = CorsOptions::default().to_cors().unwrap();
//     let helmet = Shield::default();
//     rocket::build()
//         .mount("/auth", auth::routes())
//         .mount("/api/docs", make_swagger_ui(&get_docs()))
//         .mount("/", default::routes())
//         .configure(rocket::Config {
//             port: 1410,
//             ..Default::default()
//         })
//         .attach(cors)
//         .attach(helmet)
// }

// fn get_docs() -> SwaggerUIConfig {
//     SwaggerUIConfig {
//         url: "/api/openapi.json".to_string(),
//         ..Default::default()
//     }
// }

#[macro_use] extern crate rocket;

use rocket::shield::Shield;
use rocket_cors::CorsOptions;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod modules;
mod shared; 

use modules::auth;
use modules::default;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default().to_cors().unwrap();
    let helmet = Shield::default();
    rocket::build()
        .mount("/auth", auth::routes())
        .mount("/api/docs", make_swagger_ui(&get_docs()))
        .mount("/", default::routes())
        .configure(rocket::Config {
            port: 1410,
            ..Default::default()
        })
        .attach(cors)
        .attach(helmet)
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/auth/openapi.json".to_string(), // url: "/openapi.json".to_string(),
        ..Default::default()
    }
}