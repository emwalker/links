DATABASE_FILE=./data/development.db
DATABASE_URL=sqlite:$(DATABASE_FILE)

build:
	cargo build --release

check:
	cargo clippy --fix --allow-dirty --allow-staged
	cargo fmt
	cargo test

dev:
	cargo run --bin api

prod:
	./target/release/api

setup:
	cargo install sqlx-cli
	rm -rf $(DATABASE_FILE)
	DATABASE_URL=$(DATABASE_URL) sqlx database create
	DATABASE_URL=$(DATABASE_URL) sqlx migrate run
