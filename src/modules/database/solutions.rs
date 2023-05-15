pub fn get_solutions(connection: &PgConnection) -> QueryResult<Vec<Solution>>  {
    solutions.limit(5)
        .load::<Solution>(&*connection)
}