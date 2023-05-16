// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[get("/healthz")]
// fn get_health() -> &'static str {
//     "Ok!"
// }


// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//     .mount("/", routes![index, get_health])
// }


extern crate rust_backend;

fn main() {
    rust_backend::init();
}