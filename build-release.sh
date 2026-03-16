#!/usr/bin/env bash
# Build release: cargo-leptos (Tailwind) -> copy CSS + WASM _bg -> run server.
set -e
export APP_ENVIRONMENT="${APP_ENVIRONMENT:-production}"
export RUST_LOG="${RUST_LOG:-debug}"

cargo leptos build --release
cp target/tmp/tailwind.css target/site/pkg/my_cv_rust.css
cp target/site/pkg/my_cv_rust.wasm target/site/pkg/my_cv_rust_bg.wasm

echo "Build done. Starting server..."
exec ./target/release/my_cv_rust
