# Rust build image
FROM rust:1.74-alpine3.18

# Add dependencies
RUN apk add --no-cache musl-dev bash curl zip protobuf git

# Support code formatting
RUN rustup component add rustfmt
