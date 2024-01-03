#!/bin/sh

echo "::info::Testing..."
RUST_BACKTRACE=1 cargo test
