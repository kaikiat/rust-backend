# rust-backend
## Getting started
1. Use diesel-cli for postgress setup [link](https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104)
2. `diesel setup --database-url=postgresql://localhost:5432/leetcode`
3. `diesel migration generate create_solutions`
4. `diesel migration run`
5. `diesel print-schema > src/schema.rs` | `diesel print-schema > src/database/schema.rs`
6. Connect to psql locally psql -h localhost -d $database_name -U $username or psql -h localhost -d leetcode
7. Connect via `sudo -u postgres psql`

## Pipeline (Docker)
1. docker build --platform linux/amd64 -t rust-backend .
1. docker image build --platform linux/amd64 -t rust-backend -f . 
2. Tag if not tagged docker tag rust-backend kaikiatpoh/rust-backend
3. docker push kaikiatpoh/rust-backend
4. sudo docker logs -f kaikiatpoh/rust-backend
5. Run as container `sudo docker run -d --restart=always -p 8080:8080 kaikiatpoh/rust-backend:latest`
6. Or run as `sudo docker run -d --network=host kaikiatpoh/rust-backend:latest`
7. Further optimisation `DOCKER_BUILDKIT=0 docker buildx build --ulimit nofile=1024000:1024000 --platform linux/amd64 .`

## Pipeline (Docker-Compose)
1. docker-compose up
2. docker-compose build
3. docker tag rust-backend kaikiatpoh/rust-backend:latest
4. docker push kaikiatpoh/rust-backend:latest

## Others
1. Refer to [realworld-rust-rocket](https://github.com/TatriX/realworld-rust-rocket)
1. Refer to [rust-realworld-example-app](https://github.com/navinkumarr/rust-realworld-example-app/tree/master/src)
