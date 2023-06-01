use crate::models::solutions::{Solution, SolutionJson};
use crate::schema::solutions;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;


// #[derive(Insertable)]
// #[table_name="solutions"]
// pub struct NewSolutions {
//     pub id: i32,
//     pub title: String,
//     pub description: Option<String>,
//     pub code: String,
//     pub created_on: DateTime<Utc>,
//     pub modified_on:  Option<DateTime<Utc>>,
// }

pub fn find(conn: &mut PgConnection) -> Vec<SolutionJson> {

    let result = solutions::table
        .load::<Solution>(conn)
        .expect("Failed to load solutions");

    result
        .into_iter()
        .map(|solution| solution.attach())
        .collect()
}

// pub fn find_one(conn: &PgConnection, slug: &str, user_id: Option<i32>) -> Option<ArticleJson> {
//     let article = articles::table
//         .filter(articles::slug.eq(slug))
//         .first::<Article>(conn)
//         .map_err(|err| eprintln!("articles::find_one: {}", err))
//         .ok()?;

//     let favorited = user_id
//         .map(|id| is_favorite(conn, &article, id))
//         .unwrap_or(false);

//     Some(populate(conn, article, favorited))
// }

// #[derive(FromForm, Default)]
// pub struct FeedArticles {
//     pub limit: Option<i64>,
//     pub offset: Option<i64>,
// }

// // select * from articles where author in (select followed from follows where follower = 7);
// pub fn feed(conn: &PgConnection, params: &FeedArticles, user_id: i32) -> Vec<ArticleJson> {
//     articles::table
//         .filter(
//             articles::author.eq_any(
//                 follows::table
//                     .select(follows::followed)
//                     .filter(follows::follower.eq(user_id)),
//             ),
//         )
//         .inner_join(users::table)
//         .left_join(
//             favorites::table.on(articles::id
//                 .eq(favorites::article)
//                 .and(favorites::user.eq(user_id))),
//         )
//         .select((
//             articles::all_columns,
//             users::all_columns,
//             favorites::user.nullable().is_not_null(),
//         ))
//         .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
//         .offset(params.offset.unwrap_or(0))
//         .load::<(Article, User, bool)>(conn)
//         .expect("Cannot load feed")
//         .into_iter()
//         .map(|(article, author, favorited)| article.attach(author, favorited))
//         .collect()
// }

// pub fn favorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
//     conn.transaction::<_, diesel::result::Error, _>(|| {
//         let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
//             .set(articles::favorites_count.eq(articles::favorites_count + 1))
//             .get_result::<Article>(conn)?;

//         diesel::insert_into(favorites::table)
//             .values((
//                 favorites::user.eq(user_id),
//                 favorites::article.eq(article.id),
//             ))
//             .execute(conn)?;

//         Ok(populate(conn, article, true))
//     })
//     .map_err(|err| eprintln!("articles::favorite: {}", err))
//     .ok()
// }

// pub fn unfavorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
//     conn.transaction::<_, diesel::result::Error, _>(|| {
//         let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
//             .set(articles::favorites_count.eq(articles::favorites_count - 1))
//             .get_result::<Article>(conn)?;

//         diesel::delete(favorites::table.find((user_id, article.id))).execute(conn)?;

//         Ok(populate(conn, article, false))
//     })
//     .map_err(|err| eprintln!("articles::unfavorite: {}", err))
//     .ok()
// }

// #[derive(Deserialize, AsChangeset, Default, Clone)]
// #[table_name = "articles"]
// pub struct UpdateArticleData {
//     title: Option<String>,
//     description: Option<String>,
//     body: Option<String>,
//     #[serde(skip)]
//     slug: Option<String>,
//     #[serde(rename = "tagList")]
//     tag_list: Vec<String>,
// }

// pub fn update(
//     conn: &PgConnection,
//     slug: &str,
//     user_id: i32,
//     mut data: UpdateArticleData,
// ) -> Option<ArticleJson> {
//     if let Some(ref title) = data.title {
//         data.slug = Some(slugify(&title));
//     }
//     // TODO: check for not_found
//     let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
//         .set(&data)
//         .get_result(conn)
//         .expect("Error loading article");

//     let favorited = is_favorite(conn, &article, user_id);
//     Some(populate(conn, article, favorited))
// }

// pub fn delete(conn: &PgConnection, slug: &str, user_id: i32) {
//     let result = diesel::delete(
//         articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(user_id))),
//     )
//     .execute(conn);
//     if let Err(err) = result {
//         eprintln!("articles::delete: {}", err);
//     }
// }

// fn is_favorite(conn: &PgConnection, article: &Article, user_id: i32) -> bool {
//     use diesel::dsl::exists;
//     use diesel::select;

//     select(exists(favorites::table.find((user_id, article.id))))
//         .get_result(conn)
//         .expect("Error loading favorited")
// }

// fn populate(conn: &PgConnection, article: Article, favorited: bool) -> ArticleJson {
//     let author = users::table
//         .find(article.author)
//         .get_result::<User>(conn)
//         .expect("Error loading author");

//     article.attach(author, favorited)
// }

// pub fn tags(conn: &PgConnection) -> Vec<String> {
//     articles::table
//         .select(diesel::dsl::sql("distinct unnest(tag_list)"))
//         .load::<String>(conn)
//         .expect("Cannot load tags")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_generate_suffix() {
//         for len in 3..9 {
//             assert_eq!(generate_suffix(len).len(), len);
//         }
//     }
// }