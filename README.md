# rust-backend
## Getting started
1. Use diesel-cli for postgress setup [link](https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104)
2. `diesel setup --database-url=postgresql://localhost:5432/leetcode`
3. `diesel migration generate <NAME_OF_MIGRATION>`
4. `diesel migration run`
4. `diesel migration redo` if errors
5. `diesel print-schema > src/schema.rs`
6. Connect to psql locally psql -h localhost -d $database_name -U $username or psql -h localhost -d leetcode -U postgres
7. Connect via `sudo -u postgres psql`
8. curl http://54.169.58.242:8080/healthz

## Pipeline (Docker)
1. Run `cargo build --release --bin rust-backend` first.
2. docker build --platform linux/amd64 -t rust-backend .
3. Tag if not tagged docker tag rust-backend kaikiatpoh/rust-backend
4. docker push kaikiatpoh/rust-backend
5. sudo docker logs -f kaikiatpoh/rust-backend
6. Run as container `sudo docker run -d --restart=always -p 8080:8080 kaikiatpoh/rust-backend:latest`
7. Or run as `sudo docker run -d --network=host kaikiatpoh/rust-backend:latest`
8. Further optimisation `DOCKER_BUILDKIT=0 docker buildx build --ulimit nofile=1024000:1024000 --platform linux/amd64 .`

## Database
1. Migation. Use command below
```
pg_dump -d -t solutions -U postgres leetcode > leetcode.sql
```
2. Next use this tool to convert to INSERT statement
```
cat leetcode.sql | pg-dump2insert
```

## Others
1. Refer to [realworld-rust-rocket](https://github.com/TatriX/realworld-rust-rocket)
1. Refer to [rust-realworld-example-app](https://github.com/navinkumarr/rust-realworld-example-app/tree/master/src)
