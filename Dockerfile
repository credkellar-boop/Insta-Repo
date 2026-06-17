# Build Stage
FROM rust:1.76-alpine as builder
WORKDIR /usr/src/insta-repo
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

# Runtime Stage
FROM alpine:3.19
WORKDIR /app
COPY --from=builder /usr/src/insta-repo/target/release/insta-repo /usr/local/bin/insta-repo
# Set the entrypoint so it runs like a native CLI command
ENTRYPOINT ["insta-repo"]
CMD ["--help"]
