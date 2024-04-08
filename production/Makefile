surrealdb_path := file://database
surrealdb_log_level := # --log trace
path := /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos
bin := rust-minio-s3-surreal-db-directory-traversal-uploader

stack-up:
	docker-compose up -d

stack-down:
	docker-compose down

surrealdb:
	@surreal start --auth --user root --pass root --bind 0.0.0.0:8000 ${surrealdb_path} ${surrealdb_log_level}

start-traversal:
	time ./${bin} -p ${path} > start-traversal.log

count-all:
	find ${path} | wc -l

count-files:
	find ${path} -type f | wc -l

count-dirs:
	find ${path} -type d | wc -l
