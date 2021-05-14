#!/bin/bash

if [ ! -e /tmp/www ]; then
	ln -s "$PWD/frontend/build" /tmp/www
fi

frontend_dir="$PWD/frontend"
backend_dir="$PWD/backend"

cd $frontend_dir && npm run build

if [ -e "$backend_dir/target/release/backend" ]; then
	echo "[*] Running release"
	RUST_LOG=debug "$backend_dir/target/release/backend" 
else
	echo "[*] Running debug"
	cd $backend_dir && RUST_LOG=debug cargo run
fi
