#!/bin/bash

if [ ! -e /tmp/www ]; then
	ln -s "$PWD/frontend/build" /tmp/www
fi

frontend_dir="$PWD/frontend"
backend_dir="$PWD/backend"

cd $frontend_dir && npm run build
cd $backend_dir && RUST_LOG=debug cargo run
