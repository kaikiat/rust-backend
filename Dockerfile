FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN rustup default nightly
# Build dependencies - this is the caching Docker layer!
RUN cargo +nightly chef cook --recipe-path recipe.json
# Build application
COPY . .

RUN cargo build --release --bin rust-backend

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/rust-backend /usr/local/bin

RUN apt-get update && apt-get install -y libpq5
ENV DATABASE_URL=postgresql://localhost:5432/leetcode
ENV PORT=8080

ENTRYPOINT ["/usr/local/bin/rust-backend"]