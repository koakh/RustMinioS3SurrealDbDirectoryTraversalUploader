#!/bin/bash

# Usage: ./s3upload.sh my-bucket my-file.zip
# Usage: ./s3upload.sh my-bucket resource/path/my-file.zip

bucket=$1
file=$2
# used as prefix
key=$3
# [ $key ] && key="/${key}"

if [ ! "${bucket}" ] || [ ! "${file}" ]; then
  printf "error! miss required arguments bucker or file\n$(basename ${0}) bucker file key\nnote: key is optional\nex $(basename ${0}) default-bucket ./pack.tgz /root/dir1\n"
  exit 1
fi

# host=localhost:9000
host=192.168.1.52:9000
s3_key=rust-auth
s3_secret=NTZjZGQwNzg3MDg1MzI4MWUxYTJiZTFk

resource="/${bucket}/${resource}${file}"
content_type="application/octet-stream"
date=`date -R`
_signature="PUT\n\n${content_type}\n${date}\n${resource}"
signature=`echo -en ${_signature} | openssl sha1 -hmac ${s3_secret} -binary | base64`

printf "creating resource -> http://${host}${resource}\n"

curl -X PUT -T "${file}" \
  -H "Host: ${host}" \
  -H "Date: ${date}" \
  -H "Content-Type: ${content_type}" \
  -H "Authorization: AWS ${s3_key}:${signature}" \
  http://${host}${resource}