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
