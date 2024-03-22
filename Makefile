path := upload
local_production_path := production/
remote_production_path := 192.168.1.1:/mnt/4tbdisk1/srv/docker/koakh/@RustActixWebAndSvelteKitAuthenticationDevToTutorial/
release_file := target/release/rust-minio-s3-surreal-db-directory-traversal-uploader

surrealdb:
	@surreal start --auth --log trace --user root --pass root --bind 0.0.0.0:8000 file://database.db

surrealdb-mem:
	@surreal start memory -A --auth --user root --pass root

run:
	@cargo run -- -p ${path}

run-pipe:
	@cargo run -- -p ${path} > upload.txt

build-release:
	@cargo build --release

push-release-production:
	@rsync -r -t -p -o -g -v --progress --delete -l -H -z -s  ${release_file} ${remote_production_path}

bring-prod-files:
	@rsync -r -t -p -o -g -v --progress --delete -l -H -z -s \
		--exclude volumes/minio \
		--exclude volumes/minio-old \
		--exclude rust-minio-s3-surreal-db-directory-traversal-uploader \
		${remote_production_path} ${local_production_path}