# Rust build image
FROM rust:1.74-alpine3.18

# Add dependencies
RUN apk add --no-cache \
    bash \
    curl \
    git \
    npm \
    musl-dev \
    protobuf \
    zip

# Add Rust tools
RUN rustup component add \
    clippy \
    rustfmt \
    llvm-tools

# Add Buf CLI for protobuf management
RUN npm install --global @bufbuild/buf
