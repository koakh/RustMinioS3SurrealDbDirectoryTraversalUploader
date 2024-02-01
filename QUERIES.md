# QUERIES

## Count Nodes

```sql
SELECT count() FROM nodes GROUP ALL;
```

## Count Node Types

```sql
SELECT node_type,count() FROM nodes GROUP BY node_type;
```

## Check Tree

```sql
LET $dir = "upload";
-- LET $dir = "dir1";
-- LET $dir = "dir1.1";
-- LET $dir = "dir1.1.1";
-- LET $parent_id = nodes:root;
LET $parent_id = (SELECT * FROM ONLY nodes WHERE name = $dir).id;
-- RETURN $parent_id;
SELECT 
  id, node_type, name, path, canonical_path, parent_id, s3_url
FROM 
  nodes
WHERE
  parent_id = $parent_id
ORDER BY
  name
;
```
