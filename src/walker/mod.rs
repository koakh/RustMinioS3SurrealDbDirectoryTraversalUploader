use chrono::{DateTime, Utc};
use from_os_str::try_from_os_str;
use serde::{Deserialize, Serialize};
use sha256::try_digest;
use surrealdb::sql::{Datetime, Thing};
// use sqlx::FromRow;
use std::{
  collections::HashMap,
  fmt::Display,
  fs::{self, read_link, Metadata},
  path::Path,
};
use walkdir::{DirEntry, WalkDir};

use from_os_str::Wrap;
use from_os_str::*;

use crate::{surrealdb::Database, Args, Result};

/// stores directory paths, used to get node parent path <path, uuid>
// type ParentPathHashMap = HashMap<String, Uuid>;
type ParentPathHashMap = HashMap<String, Thing>;

#[derive(Debug, Serialize, Deserialize)]
enum NodeType {
  Unknown,
  Dir,
  File,
  Symlink,
}

impl Display for NodeType {
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

#[allow(dead_code)]
// #[derive(Debug, FromRow, Deserialize, Serialize)]
#[derive(Debug, Deserialize, Serialize)]
struct Node {
  id: Thing,
  node_type: NodeType,
  name: String,
  path: String,
  // used to get real path from symlink
  canonical_path: Option<String>,
  size: u64,
  created: Datetime,  //DateTime<Utc>,
  modified: Datetime, //DateTime<Utc>,
  accessed: Datetime, //DateTime<Utc>,
  sha256: Option<String>,
  parent_id: Thing,
  notes: Option<String>,
}

// impl Node {
//   async fn save(&self, db: &Database) -> Result<surrealdb::Response> {
//     // println!("Node: {:#?}", &self);
//     // println!("Node.id: {}", &self.id);
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
//       created_at: $created_at,
//     };";
//     let mut response = db
//       .client
//       .query(sql)
//       .bind(("table", "nodes"))
//       // WIP:
//       // .bind(("id", id))
//       // .bind(("id", surrealdb::sql::Uuid(self.uuid)))
//       // .bind(("id", surrealdb::sql::Uuid(uuid::Uuid::parse_str(self.uuid.to_string().as_str()).unwrap())))
//       // .bind(("id", surrealdb::sql::Uuid(uuid::Uuid::parse_str("95f88347-02d0-472e-9851-7e67be5081c9").unwrap())))
//       .bind(&self)
//       .await?;
//     Ok(response.check()?)
//     // let errors = response.take_errors();
//     // if errors.len() > 0 {
//     //   eprintln!("errors: {:#?}", errors);
//     //   Err(format!("errors: {:#?}", errors).into())
//     // } else {
//     //   Ok(response)
//     // }
//     // WIP:
//     // let record: Option<Node> = response.take(0)?;
//     // println!("record: {:#?}", record);
//   }
// }

impl Node {
  async fn save(&self, db: &Database) -> Result<surrealdb::Response> {
    let sql = "CREATE type::table($table) CONTENT {
      node_type: $node_type,
      name: $name,
      path: $path,
      canonical_path: $canonical_path,
      size: $size,
      created: $created,
      modified: $modified,
      accessed: $accessed,
      sha256: $sha256,
      parent_id: $parent_id,
      notes: $notes,
      published: $published,
      created_at: $created_at
    };";
    let response = db
      .client
      .query(sql)
      .bind(("table", "nodes"))
      .bind(&self)
      .await?
      // check if have errors
      .check()?;
    Ok(response)
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

// TODO: move to utils
// https://stackoverflow.com/questions/64146345/how-do-i-convert-a-systemtime-to-iso-8601-in-rust
fn _iso8601_to_string(st: &std::time::SystemTime) -> String {
  let dt: DateTime<Utc> = st.clone().into();
  format!("{}", dt.format("%+"))
  // formats like "2001-07-08T00:34:60.026490+09:30"
}

// TODO: move to utils
fn iso8601(st: &std::time::SystemTime) -> DateTime<Utc> {
  let dt: DateTime<Utc> = st.clone().into();
  // output 2024-01-29T23:04:25.302137521Z
  dt
  // output 2022-07-03T07:18:52.841147Z (surrealdb format)
  // let formatted_date = dt.format("%Y-%m-%dT%H:%M:%S%.6fZ");
  // formatted_date
}

/// init walker traverse directory
pub async fn process_dirs(args: &Args, db: &Database) -> Result<()> {
  // println!("traverse paths {}", &args.path);
  let mut nodes = 0;
  let mut parent_path_hash_map = ParentPathHashMap::new();
  let walker = WalkDir::new(&args.path)
    .follow_links(true)
    .sort_by_file_name()
    .into_iter();
  for entry in walker.filter_entry(|e| !is_hidden(e)) {
    nodes += 1;
    match entry {
      Ok(v) => {
        let id: Thing = Thing {
          tb: "nodes".into(),
          id: surrealdb::sql::Id::rand(),
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
        let sha256 = match try_digest(input_path) {
          Ok(v) => Some(v),
          Err(_) => None,
        };

        // first dir is always a dir, we assign it the first id
        match node_type {
          NodeType::Unknown => {}
          NodeType::Dir => {
            // insert a key only if it doesn't already exist
            parent_path_hash_map
              .entry(v.path().display().to_string())
              .or_insert(id);
          }
          NodeType::File => {}
          NodeType::Symlink => canonical_path = Some(read_link(v.path()).unwrap().display().to_string()),
        }

        // always get path from hashmap, to use it with same id and path
        let mut current_parent_from_hash_path: String = String::default();
        // let mut current_parent_from_hash_id: Uuid = Uuid::default();
        let mut current_parent_from_hash_id: Thing = Thing {
          tb: "nodes".into(),
          id: surrealdb::sql::Id::rand(),
        };
        // override defaults
        if let Some(v) = parent_path_hash_map.get_key_value(&input_path_parent) {
          current_parent_from_hash_path = (*v.0.clone()).to_string();
          current_parent_from_hash_id = v.1.clone();
        }
        // if !current_parent_from_hash_path.starts_with("/") {
        //   current_parent_from_hash_path = current_parent_from_hash_path.replace(&args.path, "/")
        // }
        let id: Thing = Thing {
          tb: "nodes".into(),
          id: surrealdb::sql::Id::rand(),
        };
        let node = Node {
          id,
          node_type,
          name,
          canonical_path,
          path: format!("/{current_parent_from_hash_path}"),
          size: metadata.len(),
          created: surrealdb::sql::Datetime(iso8601(&metadata.created().unwrap())),
          modified: surrealdb::sql::Datetime(iso8601(&metadata.modified().unwrap())),
          accessed: surrealdb::sql::Datetime(iso8601(&metadata.accessed().unwrap())),
          sha256,
          parent_id: current_parent_from_hash_id,
          notes: None,
        };

        match node.save(&db).await {
          Ok(n) => println!("node saved: {:?}", n),
          Err(e) => eprintln!("error saving node: {:#?}", e),
        };

        // exclude root node
        if nodes > 1 {
          println!("#{} path: {}", nodes - 1, v.path().display());
          println!("\tname: {}, path: {}, node_type: {}, parent_id: {}\n", node.name, node.path, node.node_type, node.parent_id);
          println!(
            "{:#?}\n---------------------------------------------------------------------------------------------------------------------------------------------------",
            node
          );
        }
      }
      Err(e) => println!("Error: {}", e),
    }
  }

  // debug final parent_path_hash_map
  println!("parent_path_hash_map: {:#?}\n", parent_path_hash_map);
  Ok(())
}
