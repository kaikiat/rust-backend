use crate::config::DATE_FORMAT;
// use diesel::sql_types::Date;
use serde::Serialize;
// use chrono::NaiveDateTime;
use chrono::{NaiveDateTime, DateTime, Utc, NaiveDate, NaiveTime};

#[derive(Queryable)]
pub struct Solution {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub created_on: NaiveDateTime,
    pub modified_on: Option<NaiveDateTime>,
}


// pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

fn convert_string_to_datetime(string: String) -> Result<DateTime<Utc>, chrono::ParseError> {
    if string.is_empty() {
        let naive_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let datetime = DateTime::<Utc>::from_utc(naive_datetime, Utc);
        return Ok(datetime);
    }

    let naive_datetime = NaiveDateTime::parse_from_str(&string, DATE_FORMAT)?;
    let datetime = DateTime::<Utc>::from_utc(naive_datetime, Utc);
    Ok(datetime)
}

impl Solution {
    pub fn attach(self) -> SolutionJson {

        let created_on: String = self.created_on.format(DATE_FORMAT).to_string();
        let created_on_datetime = convert_string_to_datetime(created_on).unwrap();

        let modified_on = if let Some(modified_on) = self.modified_on {
            modified_on.format(DATE_FORMAT).to_string()
        } else {
            String::new()
        };
        let modified_on_datetime = convert_string_to_datetime(modified_on).unwrap();

        SolutionJson {
            id: self.id,
            title: self.title,
            description: if let Some(description) = self.description { description } else { String::new() },
            code: self.code,
            // created_on: self.created_on.format(DATE_FORMAT).to_string(),
            // modified_on: if let Some(modified_on) = self.modified_on {
            //     modified_on.format(DATE_FORMAT).to_string()
            // } else {
            //     String::new()
            // },
            created_on: created_on_datetime,
            modified_on: modified_on_datetime,
        }
    }
}

#[derive(Serialize,Debug)]
pub struct SolutionJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub code: String,
    // pub created_on: String,
    // pub modified_on: String,
    pub created_on: DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
}