path := upload-duplicated
# path := /mnt/koakhserver/mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos
# path := /mnt/koakhserver/mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/AcervosImportFolder/
local_production_path := production/
remote_production_path := 192.168.1.1:/mnt/4tbdisk1/srv/docker/koakh/@RustActixWebAndSvelteKitAuthenticationDevToTutorial/
release_file := target/release/rust-minio-s3-surreal-db-directory-traversal-uploader
surrealdb_path := file://database
surrealdb_log_level := # --log trace

stack-up:
	@cd production && docker-compose up -d

stack-down:
	@cd production && docker-compose down

surrealdb:
	@surreal start --auth --user root --pass root --bind 0.0.0.0:8000 ${surrealdb_path} ${surrealdb_log_level}

surrealdb-mem:
	@surreal start --auth --user root --pass root memory -A ${surrealdb_log_level}

run:
	@cargo run -- -p ${path}

run-pipe:
	@cargo run -- -p ${path} > upload.txt

build-release:
	@cargo build --release

push-release-production:
	@make build-release && \
	rsync -r -t -p -o -g -v --progress --delete -l -H -z -s  ${release_file} ${remote_production_path}

bring-production-files:
	@rsync -r -t -p -o -g -v --progress --delete -l -H -z -s \
		--exclude .bak \
		--exclude .trash \
		--exclude volumes/minio \
		--exclude volumes/minio-old \
		--exclude rust-minio-s3-surreal-db-directory-traversal-uploader \
		${remote_production_path} ${local_production_path}