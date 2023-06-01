use diesel::sql_types::Text;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "solutions"]
pub struct Solution {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub code: Text,
    pub body: String,
    pub created_on: NaiveDateTime,
    pub modified_on: Option<NaiveDateTime>,
}


impl Solution {
    pub fn to_solution(self) -> SolutionJson {
        SolutionJson {
            id: self.id,
            title: self.title,
            description: self.description,
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
    pub code: diesel::sql_types::Text,
    pub body: String,
    pub created_on: String,
    pub modified_on: String,
}