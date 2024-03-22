path := upload

surrealdb:
	surreal start memory -A --auth --user root --pass root

surrealdb:
	surreal start --auth --log trace --user root --pass root --bind 0.0.0.0:8000 file://database.db

run:
	cargo run -- -p ${path}

run-pipe:
	cargo run -- -p ${path} > upload.txt

build-release:
	cargo build --release