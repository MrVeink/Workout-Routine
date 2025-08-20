#!/bin/bash

# Install required tools
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the frontend
echo "Building frontend..."
wasm-pack build --target web --out-dir static/pkg

# Copy static files and create a simple server setup
echo "Frontend built successfully!"
echo "Static files are in: frontend/static/"
echo "To serve locally, you can use: python -m http.server 8000 --directory frontend/static"