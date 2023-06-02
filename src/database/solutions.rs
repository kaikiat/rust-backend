use crate::models::solutions::{Solution, SolutionJson};
use crate::schema::solutions;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};



#[derive(Insertable)]
#[table_name="solutions"]
struct NewSolution<'a> {
    title: &'a str,
    description: Option<&'a str>,
    code: &'a str,
    created_on: &'a NaiveDateTime,
}

pub fn create(conn: &mut PgConnection, title: &str, description: Option<&str>, code: &str) -> SolutionJson {
    let new_solution = &NewSolution {
        title,
        description,
        code,
        created_on: &Local::now().naive_utc(),
    };
    diesel::insert_into(solutions::table)
        .values(new_solution)
        .get_result::<Solution>(conn)
        .expect("Failed to add solution")
        .attach()
}


pub fn find(conn: &mut PgConnection) -> Vec<SolutionJson> {
    let result = solutions::table
        .load::<Solution>(conn)
        .expect("Failed to load solutions");

    result
        .into_iter()
        .map(|solution| solution.attach())
        .collect()
}