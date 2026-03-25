#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"

if ! command -v wasm-pack &>/dev/null; then
  echo "wasm-pack not found. Install with: cargo install wasm-pack"
  exit 1
fi

echo "Building WASM..."
wasm-pack build --target web --out-dir pkg --release

echo ""
echo "Done! Serve this directory and open index.html:"
echo "  python3 -m http.server 8080 --directory $(pwd)"
echo "  open http://localhost:8080"
