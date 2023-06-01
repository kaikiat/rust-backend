use crate::database::{self, Db};
use rocket::serde::json::{json, Value};

#[get("/solutions")]
pub async fn get_solutions(db: Db) -> Value {
    let solutions = db
        .run(move |conn| database::solutions::find(conn))
        .await;
    println!("Solutions : {:?}",solutions);
    json!({ "solutions": solutions})
}