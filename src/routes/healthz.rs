
use rocket::serde::json::{json, Value};

#[get("/healthz")]
pub async fn get_healthz() -> Value {
    json!({ "message": "Ok"})
}