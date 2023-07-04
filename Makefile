.PHONY: docker build

docker:
	docker build --platform linux/amd64 -t rust-backend .
	docker tag rust-backend kaikiatpoh/rust-backend
	docker push kaikiatpoh/rust-backend

build:
	rustup target add x86_64-unknown-linux-gnu
	 RUST_BACKTRACE=1 cargo build --target=x86_64-unknown-linux-gnu