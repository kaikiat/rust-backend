use crate::config::DATE_FORMAT;
use serde::{Serialize};
use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Solution {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub body: String,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
}


impl Solution {
    pub fn to_solution(self) -> SolutionJson {
        SolutionJson {
            id: self.id,
            title: self.title,
            description: if let Some(description) = self.description { description } else { String::new() },
            code: self.code,
            body: self.body,
            created_on: self.created_on.format(DATE_FORMAT).to_string(),
            modified_on: self.modified_on.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct SolutionJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub code: String,
    pub body: String,
    pub created_on: String,
    pub modified_on: String,
}