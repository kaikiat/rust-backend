#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "solutions"]
pub struct Solutions {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub code: Text,
    pub body: String,
    pub created_on: NaiveDateTime,
    pub modified_on: Option<NaiveDateTime>,
}#[derive(Insertable, Serialize, Deserialize)]

#[table_name="solutions"]
pub struct NewSolutions {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub code: Text,
    pub body: String,
    pub created_on: NaiveDateTime,
    pub modified_on: Option<NaiveDateTime>,
}