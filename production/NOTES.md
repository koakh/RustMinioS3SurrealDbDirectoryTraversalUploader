# NOTES

1. start stack and create minio bucket `default-bucket`, enable Versioning, add "Add Anonymous Access Rule" name : anonymous to bucket (in bucket anonymous side tab)
2. create user `rust-auth`
   <http://192.168.1.1:9001/identity/users/add-user>
   `.env`
     `S3_ACCESS_KEY_ID=rust-auth`
       with `readwrite access` and password
     `S3_SECRET_ACCESS_KEY=NTZjZGQwNzg3MDg1MzI4MWUxYTJiZTFk`
3. run traversal-uploader
4. imported summary

## Final Import

2024-04-03 21:26:12

```json
{
    "count": 7226
}
```

```shell
start uploading: /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos/Vídeos/Vale do Amanhecer - Entrevista com o Adj Alufã - Mestre Barro.mp4, key: Vídeos/Vale do Amanhecer - Entrevista com o Adj Alufã - Mestre Barro.mp4
node saved: node.type: file, node_id: bm9wsk982rsx6bdd0g1o, node.id: storage_node:bm9wsk982rsx6bdd0g1o
#7225 path: /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos/Vídeos/Vale do Amanhecer - Entrevista com o Adj Alufã - Mestre Barro.mp4
        name: Vale do Amanhecer - Entrevista com o Adj Alufã - Mestre Barro, path: /Vídeos, node_type: file, id: storage_node:bm9wsk982rsx6bdd0g1o, parent_id: storage_node:5zem0e35fkd2z77lhl9w
```
