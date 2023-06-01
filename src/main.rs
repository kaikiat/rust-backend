extern crate rust_backend;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rust_backend::rocket().launch().await
}