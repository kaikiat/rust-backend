use crate::config::DATE_FORMAT;
// use diesel::helper_types::Nullable;
use serde::{Serialize};
use chrono::NaiveDateTime;
// use chrono::{DateTime, Utc, NaiveDateTime};


// id -> Int4,
// title -> Varchar,
// description -> Nullable<Varchar>,
// code -> Varchar,
// created_on -> Timestamp,
// modified_on -> Nullable<Timestamp>,

#[derive(Queryable)]
pub struct Solution {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub created_on: NaiveDateTime,
    pub modified_on: Option<NaiveDateTime>,
}


impl Solution {
    pub fn attach(self) -> SolutionJson {
        SolutionJson {
            id: self.id,
            title: self.title,
            description: if let Some(description) = self.description { description } else { String::new() },
            code: self.code,
            created_on: self.created_on.format(DATE_FORMAT).to_string(),
            modified_on: if let Some(modified_on) = self.modified_on {
                modified_on.format(DATE_FORMAT).to_string()
            } else {
                String::new()
            },
        }
    }
}

#[derive(Serialize,Debug)]
pub struct SolutionJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub code: String,
    pub created_on: String,
    pub modified_on: String,
}