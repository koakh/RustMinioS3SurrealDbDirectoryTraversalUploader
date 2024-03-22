path := /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos
bin := rust-minio-s3-surreal-db-directory-traversal-uploader

stack-up:
	docker-compose up -d

stack-down:
	docker-compose down

surrealdb:
	surreal start --auth --user root --pass root --bind 0.0.0.0:8000 file://database-mar

start-traversal:
	time ./${bin} -p ${path}
