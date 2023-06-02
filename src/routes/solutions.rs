use crate::{database::{self, Db}};
use rocket::serde::json::{json, Value, Json};
use serde::Deserialize;
use crate::errors::{Errors, FieldValidator};

#[get("/solutions")]
pub async fn get_solutions(db: Db) -> Value {
    let solutions = db
        .run(move |conn| database::solutions::find(conn))
        .await;
    json!({ "solutions": solutions})
}


#[derive(Deserialize)]
pub struct NewSolution {
    solution: NewSolutionData,
}

#[derive(Deserialize, Validate)]
pub struct NewSolutionData {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 0))]
    description: Option<String>,
    #[validate(length(min = 1))]
    code: String,
}


#[post("/solutions", format = "application/json", data = "<new_solution>")]
pub async fn post_solution(
    new_solution: Json<NewSolution>,
    db: Db,
) -> Result<Value, Errors> {

    let new_solution = new_solution.into_inner().solution;
    let mut extractor = FieldValidator::validate(&new_solution);
    let title = extractor.extract("title", Some(new_solution.title));
    let code = extractor.extract("code", Some(new_solution.code));
    extractor.check()?;

    let solution = db
        .run(move |conn| {
            database::solutions::create(
                conn,
                &title,
                new_solution.description.as_deref(),
                &code,
            )
        })
        .await;
    Ok(json!({ "solution": solution }))
}
