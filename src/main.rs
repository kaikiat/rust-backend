#[macro_use] extern crate rocket;

mod modules;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/healthz")]
fn get_health() -> &'static str {
    "Ok!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(modules::database::connection::connect())
        .mount("/", routes![index, get_health])
        .launch();
}

// fn rocket() -> _ {
//     rocket::build()
//     .mount("/", routes![index, get_health])
// }
