.PHONY: build run test clean docker-build docker-scan

build:
	cargo build --release

run:
	cargo run -- --target .

test:
	cargo test

clean:
	cargo clean

docker-build:
	docker build -t credkellar-boop/insta-repo .

# Example of scanning a mounted directory safely inside the container
docker-scan: docker-build
	docker run --rm -v $(PWD):/scan credkellar-boop/insta-repo --target /scan
