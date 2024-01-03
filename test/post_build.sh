#!/bin/sh

echo "::info Running tests..."
RUST_BACKTRACE=1 cargo test
