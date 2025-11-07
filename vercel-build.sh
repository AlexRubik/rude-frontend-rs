#!/bin/bash
set -e

echo "Installing Rust toolchain..."
export RUSTUP_HOME=/tmp/rustup
export CARGO_HOME=/tmp/cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
export PATH="$CARGO_HOME/bin:$PATH"

echo "Adding WebAssembly target..."
rustup target add wasm32-unknown-unknown

echo "Installing Dioxus CLI..."
curl -L --proto '=https' --tlsv1.2 -sSf https://dioxuslabs.com/dx-install.sh | sh

echo "Installing Node.js dependencies..."
npm install

echo "Building application..."
npm run build
