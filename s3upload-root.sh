#!/bin/bash

# ./s3upload.sh default-bucket root/dir3/dir3.file 
# creating resource -> http://192.168.1.52:9000/default-bucket/root/dir3/dir3.file

BUCKET="default-bucket"

for i in $(find root -type f)
do
	./s3upload.sh	${BUCKET} ${i}
done