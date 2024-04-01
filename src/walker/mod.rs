use from_os_str::try_from_os_str;
use serde::{Deserialize, Serialize};
use sha256::try_digest;
use std::{
    collections::HashMap,
    fs::{self, read_link, Metadata},
    path::Path,
};
use surrealdb::{
    opt::Resource,
    sql::{Datetime as SdbDatetime, Id as DdbId, Thing},
};
use walkdir::{DirEntry, WalkDir};

use from_os_str::Wrap;
use from_os_str::*;

use crate::{app::STORAGE_NODES_TABLE, minio::Client, surrealdb::Database, utils::st2sdt, Args, Result};

struct ParentPathProp {
    thing: Thing,
    ancestors: Option<Vec<Thing>>,
}
type ParentPathHashMap = HashMap<String, ParentPathProp>;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    Unknown,
    Dir,
    File,
    Symlink,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "unknown"),
            Self::Dir => write!(f, "dir"),
            Self::File => write!(f, "file"),
            Self::Symlink => write!(f, "symlink"),
        }
    }
}

impl From<Metadata> for NodeType {
    fn from(metadata: Metadata) -> Self {
        if metadata.is_dir() {
            Self::Dir
        } else if metadata.is_file() {
            Self::File
        } else if metadata.is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct StorageNode {
    id: Thing,
    node_type: NodeType,
    name: String,
    path: String,
    // used to get real path from symlink
    canonical_path: Option<String>,
    s3_url: Option<String>,
    size: u64,
    created: SdbDatetime,
    modified: SdbDatetime,
    accessed: SdbDatetime,
    sha256: Option<String>,
    parent_id: Thing,
    // tags: Option<Vec<T>>,
    // categories: Option<Vec<T>>,
    // mastery_levels: Option<Vec<T>>,
    notes: Option<String>,
    published: bool,
    // updated_at: Option<surrealdb::sql::Datetime>,
    // created_at: Option<surrealdb::sql::Datetime>,
}

// // using raw query
// impl Node {
//   async fn save(&self, db: &Database) -> Result<surrealdb::Response> {
//     let sql = "CREATE type::table($table) CONTENT {
//       node_type: $node_type,
//       name: $name,
//       path: $path,
//       canonical_path: $canonical_path,
//       size: $size,
//       created: $created,
//       modified: $modified,
//       accessed: $accessed,
//       sha256: $sha256,
//       parent_id: $parent_id,
//       notes: $notes,
//       published: $published,
//       created_at: $created_at
//     };";
//     let response = db
//       .client
//       .query(sql)
//       .bind(("table", "storage_node"))
//       .bind(&self)
//       .await?
//       // check if have errors
//       .check()?;
//     Ok(response)
//   }
// }

impl StorageNode {
    async fn save(&self, db: &Database) -> Result<()> {
        db.client
            .create(Resource::from(STORAGE_NODES_TABLE))
            .content(self)
            .await?;
        Ok(())
    }
}

/// check if dirEntry is hidden
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// init walker traverse directory
pub async fn process_dirs(args: &Args, db: &Database, s3_client: &Client) -> Result<()> {
    // println!("traverse paths {}", &args.path);
    let mut storage_nodes = 0;
    let mut parent_path_hash_map = ParentPathHashMap::new();
    let walker = WalkDir::new(&args.path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        storage_nodes += 1;
        match entry {
            Ok(v) => {
                let node_id;
                // if first node, use a fixed root id
                if parent_path_hash_map.len() == 0 {
                    node_id = DdbId::String("root".into())
                } else {
                    node_id = DdbId::rand();
                }
                let id: Thing = Thing {
                    tb: STORAGE_NODES_TABLE.into(),
                    id: node_id.clone(),
                };
                let metadata = fs::symlink_metadata(v.path())?;
                let node_type: NodeType = metadata.clone().into();
                let input_path = Path::new(v.path());
                let os_str = v.file_name();
                // filename or directory name
                let name = try_from_os_str!(os_str as &Path)
                    .unwrap()
                    .display()
                    .to_string();
                // the path without filename from input path
                let input_path_parent = input_path.parent().unwrap().display().to_string();
                // this will work with paths with a middle dir symlink ex `root/dir2/dir1.2.1.link/dir1.2.1.file`, else this path will be "/" and not "root/dir2/dir1.2.1.link"
                // let input_path_parent = input_path.parent().unwrap().display().to_string().replace(name.as_str(), "");
                let mut canonical_path: Option<String> = None;
                // TODO: only if is a File
                let sha256 = match try_digest(input_path) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };

                // first inter is always a dir, we assign it the first id
                match node_type {
                    NodeType::Unknown => {}
                    NodeType::Dir => {
                        // insert a key only if it doesn't already exist
                        parent_path_hash_map
                            .entry(v.path().display().to_string())
                            .or_insert(ParentPathProp { thing: id.clone(), ancestors: None });
                    }
                    NodeType::File => {}
                    NodeType::Symlink => canonical_path = Some(read_link(v.path()).unwrap().display().to_string()),
                }

                // always get path from hashmap, to use it with same id and path
                let mut current_parent_from_hash_path: String = String::default();
                // let mut current_parent_from_hash_id: Uuid = Uuid::default();
                let mut current_parent_from_hash_id: Thing = Thing {
                    tb: STORAGE_NODES_TABLE.into(),
                    id: DdbId::rand(),
                };
                // override defaults
                if let Some(v) = parent_path_hash_map.get_key_value(&input_path_parent) {
                    current_parent_from_hash_path = (*v.0.clone()).to_string();
                    current_parent_from_hash_id = v.1.thing.clone();
                }
                // remove root (source path) from final path, and assign / to it
                let path;
                if !current_parent_from_hash_path.eq(&args.path) {
                    let replace = format!("{}", &args.path);
                    // println!("replace: {}, current_parent_from_hash_path: {}", replace, current_parent_from_hash_path);
                    path = current_parent_from_hash_path.replace(&replace, "");
                } else {
                    path = "/".into();
                }

                let mut s3_url: Option<String> = None;
                if node_type == NodeType::File {
                    let upload_file = format!("{}/{}", &current_parent_from_hash_path, &name);
                    // always remove root args path from key, and start slash
                    let key = &upload_file.replace(&args.path, "")[1..];
                    // key must be equal to file path without root path in this case is upload_file ex '/root.file'
                    println!("start uploading: {}, key: {}", &upload_file, &key);
                    // always remove base endpoint from
                    let (_, s3_bucket_name_key) = s3_client.put_object_from_file(&upload_file, &key).await;
                    // override default
                    s3_url = Some(s3_bucket_name_key);
                }

                let node = StorageNode {
                    id,
                    node_type: node_type.clone(),
                    name: name.clone(),
                    canonical_path,
                    path: path.clone(),
                    size: metadata.len(),
                    created: st2sdt(&metadata.created().unwrap()),
                    modified: st2sdt(&metadata.modified().unwrap()),
                    accessed: st2sdt(&metadata.accessed().unwrap()),
                    sha256,
                    parent_id: current_parent_from_hash_id,
                    s3_url: s3_url.clone(),
                    notes: None,
                    published: false,
                };

                match node.save(&db).await {
                    Ok(_) => {
                        println!("node saved: node.type: {}, node_id: {}, node.id: {}:{}", &node.node_type, &node_id, &node.id.tb, &node.id.id);
                    }
                    Err(e) => eprintln!("error saving node: {:#?}", e),
                };

                println!("#{} path: {}", storage_nodes - 1, v.path().display());
                println!(
                    "\tname: {}, path: {}, node_type: {}, id: {}:{}, parent_id: {}\n",
                    &node.name, &node.path, &node.node_type, &node.id.tb, &node.id.id, &node.parent_id
                );
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    // debug final parent_path_hash_map
    // println!("parent_path_hash_map: {:#?}\n", parent_path_hash_map);
    // parent_path_hash_map
    //   .into_iter()
    //   .for_each(|(k, v)| println!("k: {} -> v: {}", k, v));

    Ok(())
}
