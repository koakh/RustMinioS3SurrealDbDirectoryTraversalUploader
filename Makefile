surrealdb:
	surreal start memory -A --auth --user root --pass root

run:
	cargo run -- -p root > root.txt

# db-create:
# 	sqlx database create

# db-drop:	
# 	sqlx database drop

# db-up:
# 	sqlx migrate run

# db-down:
# 	sqlx migrate revert