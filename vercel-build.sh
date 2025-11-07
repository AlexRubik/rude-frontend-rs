#!/bin/bash
set -e

echo "Installing Rust toolchain..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

echo "Adding WebAssembly target..."
rustup target add wasm32-unknown-unknown

echo "Installing Dioxus CLI..."
curl -L --proto '=https' --tlsv1.2 -sSf https://dioxuslabs.com/dx-install.sh | sh

echo "Installing Node.js dependencies..."
npm install

echo "Building application..."
npm run build
