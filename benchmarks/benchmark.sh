#!/bin/sh

ACTIX_PORT=8080
AXUM_PORT=8082
WARP_PORT=8083

echo "Installing node modules..."
npm install

echo "Building binaries..."
cargo build --release

echo "Running binaries..."
./target/release/actix &
disown
./target/release/axum &
disown
./target/release/warp &
disown

sleep 3

printf "\n\nActix:\n"
wrk -t1 -c1 -d60s "http://127.0.0.1:$ACTIX_PORT"

printf "\n\Warp:\n"
wrk -t1 -c1 -d60s "http://127.0.0.1:$WARP_PORT"

printf "\n\nAxum:\n"
wrk -t1 -c1 -d60s "http://127.0.0.1:$AXUM_PORT"

pkill -c -9 actix >/dev/null
pkill -c -9 warp >/dev/null
pkill -c -9 axum >/dev/null
