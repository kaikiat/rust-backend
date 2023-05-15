# rust-backend
1. Use diesel-cli for postgress setup [link](https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104)
2. `diesel setup --database-url=postgresql://localhost:5432/leetcode`
3. `diesel migration generate create_solutions`
4. `diesel migration run`
5. `diesel print-schema > src/schema.rs` | `diesel print-schema > src/database/schema.rs`
6. Connect to psql locally psql -h localhost -d $database_name -U $username or psql -h localhost -d leetcode

## Others
1. Refer to [link](https://github.com/TatriX/realworld-rust-rocket)
1. Refer to [link](https://github.com/navinkumarr/rust-realworld-example-app/tree/master/src)