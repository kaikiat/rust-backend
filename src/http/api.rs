use rocket;
use rocket::Rocket;
use http::errors::handlers::*;

use http::resources::solutions::handlers::*;

// pub fn main(settings: Settings) {
//     rocket(settings).launch();
// }
pub fn main() {
    rocket().launch();
}


// pub fn rocket(settings: Settings) -> Rocket {
pub fn rocket() -> Rocket {
    rocket::ignite()
        // .manage(init_db(&settings.database))
        // .manage(settings)
        // .mount("/api/users", routes![current_user_handler, register_user_handler, login_user_handler, update_user_handler])
        // .mount("/api/articles", routes![create_article_handler])
        // .mount("/api/solutions", routes![create_solution])
        .mount("/api/solutions", routes![get_solution])
        .catch(catchers![
            not_found,
            unauthenticated,
            unauthorized,
            bad_request,
            unprocessable_entity,
            internal_server_error
        ])
}