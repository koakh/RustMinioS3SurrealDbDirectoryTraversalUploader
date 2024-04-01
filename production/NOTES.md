# NOTES

1. start stack and create minio bucket `default-bucket`, enable Versioning
2. create user `rust-auth`
   <http://192.168.1.1:9001/identity/users/add-user>
   `.env`
     `S3_ACCESS_KEY_ID=rust-auth`
       with `readwrite access` and password
     `S3_SECRET_ACCESS_KEY=NTZjZGQwNzg3MDg1MzI4MWUxYTJiZTFk`
3. run traversal-uploader
4. imported summary

start uploading: /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos/Vídeos/VALE DO AMANHECER  Entrevista com o Adj  Alufã   Mestre Barro[1].mp4, key: Vídeos/VALE DO AMANHECER  Entrevista com o Adj  Al
ufã   Mestre Barro[1].mp4
node saved: node.type: file, node_id: n8s2ger5jozzsaz3j7kb, node.id: storage_node:n8s2ger5jozzsaz3j7kb
#7339 path: /mnt/4tbdisk1/srv/docker/linuxserver/syncthing/volumes/syncthing/data1/Shared/Acervos/Vídeos/VALE DO AMANHECER  Entrevista com o Adj  Alufã   Mestre Barro[1].mp4
        name: VALE DO AMANHECER  Entrevista com o Adj  Alufã   Mestre Barro[1].mp4, path: /Vídeos, node_type: file, id: storage_node:n8s2ger5jozzsaz3j7kb, parent_id: storage_node:n41py002ed8nwjtgjkz1

296.05user 36.38system 58:51.60elapsed 9%CPU (0avgtext+0avgdata 3068704maxresident)k
93284248inputs+0outputs (312major+9382455minor)pagefaults 0swaps

minio 6304
traversal-uploader 7339
