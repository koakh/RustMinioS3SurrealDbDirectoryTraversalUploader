surrealdb_database := database
# the database that have Acervos and AcervosNew records, to get non duplicated files from AcervosNew
# surrealdb_database := database-both
# v1.5.4
# surrealdb_path := file://${surrealdb_database}
surrealdb_path := rocksdb://${surrealdb_database}
# surrealdb_path := file://database-new
surrealdb_log_level := # --log trace
# path := /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos
# path := /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/AcervosNew
# path := /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/AcervosImportFolder
path := /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/AcervosNewCombinedFoldersFinal
bin := rust-minio-s3-surreal-db-directory-traversal-uploader

stack-up:
	docker-compose up -d

stack-down:
	docker-compose down

surrealdb:
# v1.5.4
# @surreal start --auth --user root --pass root --bind 0.0.0.0:8000 ${surrealdb_path} ${surrealdb_log_level}
	@surreal start --user root --pass root --bind 0.0.0.0:8000 ${surrealdb_path} ${surrealdb_log_level}

surrealdb-update:
	@curl -sSf https://install.surrealdb.com | sh && sudo cp /home/mario/.surrealdb/surreal /usr/local/bin/surreal && surreal version

start-traversal:
	# stdout and stderr outputs to terminal and file
	# https://unix.stackexchange.com/a/639748
	time ./${bin} -p ${path} 2>&1 | tee start-traversal.log

count-all:
	find ${path} | wc -l

count-files:
	find ${path} -type f | wc -l

count-dirs:
	find ${path} -type d | wc -l

list-files:
	find ${path} -type f > listfiles.txt
#	find ${path} -name '*  *.*' -type f > listfiles.txt

list-dirs:
#        find ${path} -type f > listfiles.txt
	find ${path} -name '*  *.*' -type d > listdirs.txt

update-certificates:
	./update-certificates.sh && docker-compose down && docker-compose up -d
