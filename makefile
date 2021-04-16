
install:
	./install.sh

build-frontend:
	cd frontend && npm run build

build-backend:
	cd backend && cargo build

build: build-frontend build-backend

run: install build
	cd backend && cargo run
