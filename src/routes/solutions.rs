use crate::database::{self, Db};
// use crate::errors::{Errors, FieldValidator};
use rocket::serde::json::{json, Value};

// Just for reference
// pub struct NewSolutions {
//     pub id: i32,
//     pub title: String,
//     pub description: Option<String>,
//     pub code: Text,
//     pub body: String,
//     pub created_on: NaiveDateTime,
//     pub modified_on: Option<NaiveDateTime>,
// }

// #[post("/solutions", format = "json", data = "<new_solution>")]
// pub async fn post_solutions(
//     new_solution: Json<NewArticle>,
//     db: Db,
// ) -> Result<Value, Errors> {
//     let new_solution = new_solution.into_inner().solution;

//     let mut extractor = FieldValidator::validate(&new_solution);
//     let title = extractor.extract("title", new_article.title);
//     let description = extractor.extract("description", new_article.description);
//     let body = extractor.extract("body", new_article.body);
//     extractor.check()?;

//     let article = db
//         .run(move |conn| {
//             database::articles::create(
//                 conn,
//                 auth.id,
//                 &title,
//                 &description,
//                 &body,
//                 &new_article.tag_list,
//             )
//         })
//         .await;
//     Ok(json!({ "article": article }))
// }

// /// return multiple articles, ordered by most recent first
#[get("/solutions")]
pub async fn get_solutions(db: Db) -> Value {
    let solutions = db
        .run(move |conn| database::solutions::find(conn))
        .await;
    json!({ "articles": solutions.0, "articlesCount": solutions.1 })
}



// #[delete("/solutions/<slug>")]
// pub async fn delete_article(slug: String, auth: Auth, db: Db) {
//     db.run(move |conn| {
//         database::articles::delete(conn, &slug, auth.id);
//     })
//     .await;
// }

// #[derive(Deserialize)]
// pub struct UpdateArticle {
//     article: database::articles::UpdateArticleData,
// }

// #[put("/articles/<slug>", format = "json", data = "<article>")]
// pub async fn put_articles(
//     slug: String,
//     article: Json<UpdateArticle>,
//     auth: Auth,
//     db: Db,
// ) -> Option<Value> {
//     // TODO: check auth
//     db.run(move |conn| {
//         database::articles::update(conn, &slug, auth.id, article.into_inner().article)
//     })
//     .await
//     .map(|article| json!({ "article": article }))
// }