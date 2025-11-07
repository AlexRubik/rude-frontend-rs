#!/bin/bash
set -e

# Set up Rust environment variables
export RUSTUP_HOME=/tmp/rustup
export CARGO_HOME=/tmp/cargo
export PATH="$CARGO_HOME/bin:$PATH"

echo "Installing Rust toolchain..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

echo "Verifying Rust installation..."
which rustc
rustc --version

echo "Adding WebAssembly target..."
rustup target add wasm32-unknown-unknown

echo "Installing Dioxus CLI..."
cargo install dioxus-cli --version 0.6.3 --locked

echo "Verifying Dioxus CLI installation..."
which dx
dx --version

echo "Installing Node.js dependencies..."
npm install

echo "Building application..."
npm run build
