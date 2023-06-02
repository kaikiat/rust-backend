use crate::database::{self, Db};
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
    title: Option<String>,
    #[validate(length(min = 0))]
    description: Option<String>,
    #[validate(length(min = 1))]
    code: Option<String>,
}


#[post("/solutions", format = "application/json", data = "<new_solution>")]
pub async fn post_solution(
    new_solution: Json<NewSolution>,
    db: Db,
) -> Result<Value, Errors> {
    let new_solution = new_solution.into_inner().solution;

    let mut extractor = FieldValidator::validate(&new_solution);
    let title = extractor.extract("title", new_solution.title);
    let description = extractor.extract("description", new_solution.description);
    let code = extractor.extract("code", new_solution.code);
    extractor.check()?;


    let solution = db
        .run(move |conn| {
            database::solutions::create(
                conn,
                &title,
                Some(&description),
                &code,
            )
        })
        .await;
    Ok(json!({ "solution": solution }))
}
