DATABASE_FILE=./data/development.db
DATABASE_URL=sqlite:$(DATABASE_FILE)

check:
	cargo clippy --fix --allow-dirty --allow-staged
	cargo fmt
	cargo test

dev:
	cargo run --bin api

load-fixtures:
	rm -rf $(DATABASE_FILE)
	DATABASE_URL=$(DATABASE_URL) sqlx database create
	DATABASE_URL=$(DATABASE_URL) sqlx migrate run
