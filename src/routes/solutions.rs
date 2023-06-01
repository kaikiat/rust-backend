use crate::database::articles::{FeedArticles, FindArticles};
use crate::database::{self, Db};
use crate::errors::{Errors, FieldValidator};
use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewSolution {
    solution: NewSolutionData,
}

#[derive(Deserialize, Validate)]
pub struct NewSolutionData {
    #[validate(length(min = 1))]
    title: Option<String>,
    #[validate(length(min = 1))]
    description: Option<String>,
    #[validate(length(min = 1))]
    body: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

// pub struct NewSolutions {
//     pub id: i32,
//     pub title: String,
//     pub description: Option<String>,
//     pub code: Text,
//     pub body: String,
//     pub created_on: NaiveDateTime,
//     pub modified_on: Option<NaiveDateTime>,
// }

#[post("/solutions", format = "json", data = "<new_solution>")]
pub async fn post_solutions(
    new_solution: Json<NewArticle>,
    db: Db,
) -> Result<Value, Errors> {
    let new_solution = new_solution.into_inner().solution;

    let mut extractor = FieldValidator::validate(&new_solution);
    let title = extractor.extract("title", new_article.title);
    let description = extractor.extract("description", new_article.description);
    let body = extractor.extract("body", new_article.body);
    extractor.check()?;

    let article = db
        .run(move |conn| {
            database::articles::create(
                conn,
                auth.id,
                &title,
                &description,
                &body,
                &new_article.tag_list,
            )
        })
        .await;
    Ok(json!({ "article": article }))
}

// /// return multiple articles, ordered by most recent first
// #[get("/articles?<params..>")]
// pub async fn get_articles(params: FindArticles, auth: Option<Auth>, db: Db) -> Value {
//     let user_id = auth.map(|x| x.id);
//     let articles = db
//         .run(move |conn| database::articles::find(conn, &params, user_id))
//         .await;
//     json!({ "articles": articles.0, "articlesCount": articles.1 })
// }

#[get("/articles/<slug>")]
pub async fn get_article(slug: String, auth: Option<Auth>, db: Db) -> Option<Value> {
    let user_id = auth.map(|x| x.id);
    db.run(move |conn| database::articles::find_one(conn, &slug, user_id))
        .await
        .map(|article| json!({ "article": article }))
}

// #[delete("/articles/<slug>")]
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