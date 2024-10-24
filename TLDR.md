# TLDR

## Project Deployed on

- `koakh-server:/mnt/4tbdisk1/srv/docker/koakh/@RustActixWebAndSvelteKitAuthenticationDevToTutorial`
  - [https://minio.koakh.com:9000/](https://minio.koakh.com:9000/)
- static acervos: `/mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/AcervosNewCombinedFoldersFinal`

## Start Dev Env

```shell
$ cd /mnt/4tbdisk1/srv/docker/koakh/@RustActixWebAndSvelteKitAuthenticationDevToTutorial
$ make stack-up
 ⠿ Container rust-sveltekit-auth-starter-redis
 ⠿ Container rust-sveltekit-auth-starter-db
 ⠿ Container rust-sveltekit-auth-starter-minio3
 ⠿ Container rust-sveltekit-auth-starter-minio1
 ⠿ Container rust-sveltekit-auth-starter-minio2
 ⠿ Container rust-sveltekit-auth-starter-minio4
 ⠿ Container rust-sveltekit-auth-starter-nginx
```

read `production/NOTES.md` to configure minio

```shell
# use same path
$ cd /mnt/4tbdisk1/srv/docker/koakh/@RustActixWebAndSvelteKitAuthenticationDevToTutorial
$ make surrealdb
$ make run
```

go to vscode [development environment](kunbutu-2310-pmox:~/Development/@WhiteArrow/RustActixWebAndSvelteKitAuthenticationDevToTutorial) and run

```shell
$ cd ~/Development/@WhiteArrow/RustActixWebAndSvelteKitAuthenticationDevToTutorial
# win1
$ just stack-up
$ just server-run
# win2
$ just frontend-run
```

now open broswer at <https://192.168.1.84:3000/app/searchcms>
