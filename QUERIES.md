# QUERIES

## Count StorageNode

```sql
SELECT count() FROM storage_node GROUP ALL;
```

## Count Node Types

```sql
SELECT node_type,count() FROM storage_node GROUP BY node_type;
```

## Check Tree

```sql
LET $dir = "upload";
-- LET $dir = "dir1";
-- LET $dir = "dir1.1";
-- LET $dir = "dir1.1.1";
-- LET $parent_id = storage_node:root;
LET $parent_id = (SELECT * FROM ONLY storage_node WHERE name = $dir).id;
-- RETURN $parent_id;
SELECT 
  id, node_type, name, path, canonical_path, parent_id, s3_url
FROM 
  storage_node
WHERE
  parent_id = $parent_id
ORDER BY
  name
;
```

## Select by Ancestors

```shell
#0 path: upload
#  []
#
#1 path: upload/dir1
#  [upload]
#
#2 path: upload/dir1/dir1.1
#  [upload, dir1]
#
#3 path: upload/dir1/dir1.1/dir1.1.1
#  [upload, dir1, dir1.1]
#
#4 path: upload/dir1/dir1.1/dir1.1.1/dir1.1.1-1.file
#  [upload, dir1, dir1.1, dir1.1.1]
#
#5 path: upload/dir1/dir1.1/dir1.1.1/dir1.1.1-2.file
#  [upload, dir1, dir1.1, dir1.1.1]
#
#6 path: upload/dir1/dir1.1/dir1.1.1/dir1.1.1-3.file
#  [upload, dir1, dir1.1, dir1.1.1]
#
#7 path: upload/dir1/dir1.1/dir1.1.1/dir1.1.1-4.file
#  [upload, dir1, dir1.1, dir1.1.1]
#
#8 path: upload/dir1/dir1.1/dir1.1.1/dir1.1.1-5.file
#  [upload, dir1, dir1.1, dir1.1.1]
#
#9 path: upload/dir1/dir1.1/dir1.1.file
#  [upload, dir1, dir1.1]
#

``````sql
-- DELETE FROM storage_node;
-- SELECT count() as count FROM storage_node GROUP ALL;
SELECT 
id, name, path, parentId, ancestors 
FROM storage_node WHERE 
-- parentId = storage_node:root;
-- name = 'dir1.1.1-1.file' -- path /dir1/dir1.1/dir1.1.1
-- path = "/dir1/dir1.1/dir1.1.1"
id = storage_node:5ra68r26zyyvtgu73ccw
-- {
--     "ancestors": [
--          "storage_node:root",
--          "storage_node:5ra68r26zyyvtgu73ccw",       "path": "/"
--          "storage_node:1k4lmbz22ub9liic1kjj",       "path": "/dir1"
--          "storage_node:jeo51q86k802pt95rp5a"        "path": "/dir1/dir1.1"
--     ],
--     "id": "storage_node:736bn3cidbmhs97he5fi",
--     "name": "dir1.1.1-1.file",
--     "parentId": "storage_node:7sasl4ub6r27tdn14pq1",
--     "path": "/dir1/dir1.1/dir1.1.1"
-- },

-- select all files inside /dir1/dir1.1
let $parentId = (SELECT id FROM storage_node WHERE name = 'dir1.1')[0].id;
SELECT name, nodeType, path FROM storage_node WHERE $parentId IN ancestors AND nodeType = 'file' ORDER BY nodeType;

[
    {
        "name": "dir1.1.1-2.file",
        "nodeType": "file",
        "path": "/dir1/dir1.1/dir1.1.1"
    },
    {
        "name": "dir1.1.1-5.file",
        "nodeType": "file",
        "path": "/dir1/dir1.1/dir1.1.1"
    },
    {
        "name": "dir1.1.1-1.file",
        "nodeType": "file",
        "path": "/dir1/dir1.1/dir1.1.1"
    },
    {
        "name": "dir1.1.file",
        "nodeType": "file",
        "path": "/dir1/dir1.1"
    },
    {
        "name": "dir1.1.1-3.file",
        "nodeType": "file",
        "path": "/dir1/dir1.1/dir1.1.1"
    },
    {
        "name": "dir1.1.1-4.file",
        "nodeType": "file",
        "path": "/dir1/dir1.1/dir1.1.1"
    }
]
```

## Select Ancestors with Path String a lot Easier and Reddable

wtf: we can use traversal searschs using just the path ex, like wildcards in a file system

```sql
$ SELECT id, name, fileName, nodeType, path FROM storage_node WHERE  string::startsWith(path, '/Adjuntos/Adjunto Aluano');
```

## Check id all S3Url have a minio url, ex urls without NONE

```sql
select id, name, fileName, s3Url, fullPath, fileSize from storage_node where nodeType = 'file' and s3Url = none;
-- count
select count() as count, s3Url from storage_node where nodeType = 'file' and s3Url = none group by s3Url;
```