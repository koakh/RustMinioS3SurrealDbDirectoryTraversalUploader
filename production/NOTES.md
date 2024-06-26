# NOTES

1. start stack and create minio bucket `default-bucket`, enable Versioning
2. create user `rust-auth`
   <http://192.168.1.1:9001/identity/users/add-user>
   `.env`
     `S3_ACCESS_KEY_ID=rust-auth`
       with `readwrite access` and password
     `S3_SECRET_ACCESS_KEY=NTZjZGQwNzg3MDg1MzI4MWUxYTJiZTFk`
3. add custom policy to `default-bucket`

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "AWS": [
          "*"
        ]
      },
      "Action": [
        "s3:GetBucketLocation"
      ],
      "Resource": [
        "arn:aws:s3:::default-bucket"
      ]
    },
    {
      "Effect": "Allow",
      "Principal": {
        "AWS": [
          "*"
        ]
      },
      "Action": [
        "s3:GetObject"
      ],
      "Resource": [
        "arn:aws:s3:::default-bucket/*"
      ]
    }
  ]
}
```

4. run traversal-uploader
5. imported summary

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

