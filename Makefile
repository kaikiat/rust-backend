.PHONY: docker

docker:
	docker build --platform linux/amd64 -t rust-backend .
	docker tag rust-backend kaikiatpoh/rust-backend
	docker push kaikiatpoh/rust-backend