#!/usr/bin/env bash
set -e

# 1) ./game_core to wasm inside ./tauri-app/src/pck
cd game_core
wasm-pack build --target web --out-dir ../tauri-app/src/pkg --release --features wasm
rm -rf ../tauri-app/public/data/
cp -r data/ ../tauri-app/public/

# 2) build frontend
cd ../tauri-app
npm install
npm run build

# 3) zip ./tauri-app/dist to ./game-web.zip
cd dist
zip -r ../../game-web.zip ./*
rm -rf ../../docs/
mkdir ../../docs/
cp -r ./* ../../docs/

echo "Created ./game-web.zip"
