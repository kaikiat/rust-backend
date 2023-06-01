use rust_backend;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rust_backend::rocket().launch().await
}