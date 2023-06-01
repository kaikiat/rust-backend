#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket_sync_db_pools;

extern crate chrono;

extern crate rocket_cors;
use rocket::{Request, Response};
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use]
extern crate diesel;

use dotenv::dotenv;

mod config;
mod database;
// mod errors;
mod models;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::solutions::get_solutions,
            ],
        )
        .attach(database::Db::fairing())
        .attach(Cors)
        .register("/", catchers![not_found])
}