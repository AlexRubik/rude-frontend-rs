#!/bin/bash
set -e

# Exit on any error with detailed information
trap 'echo "Error on line $LINENO"' ERR

# Set up Rust environment variables
export RUSTUP_HOME=/tmp/rustup
export CARGO_HOME=/tmp/cargo
export PATH="$CARGO_HOME/bin:$PATH"

# Critical: Set RUSTFLAGS for WASM compilation to avoid getrandom issues
export CARGO_BUILD_TARGET="wasm32-unknown-unknown"
export RUSTFLAGS="-C link-arg=-s"

echo "=== Installing Rust toolchain ==="
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable

echo "=== Verifying Rust installation ==="
which rustc
rustc --version
cargo --version

echo "=== Adding WebAssembly target ==="
rustup target add wasm32-unknown-unknown

echo "=== Verifying WASM target ==="
rustup target list --installed | grep wasm32

echo "=== Installing Dioxus CLI (this may take 3-5 minutes) ==="
cargo install dioxus-cli --version 0.6.3 --locked

echo "=== Verifying Dioxus CLI installation ==="
which dx
dx --version

echo "=== Installing Node.js root dependencies ==="
npm install --prefer-offline --no-audit

echo "=== Building CSS ==="
npm run build:css

echo "=== Building wallet adapter ==="
cd wallet-adapter
npm install --prefer-offline --no-audit
npm run build
cd ..

echo "=== Verifying wallet.js was created ==="
ls -lh public/wallet.js

echo "=== Building Rust application to WebAssembly ==="
# Set environment to ensure getrandom and other deps use correct WASM features
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C link-arg=-s"

# Build with explicit release profile
dx build --release --platform web

echo "=== Verifying build output ==="
ls -lh dist/
