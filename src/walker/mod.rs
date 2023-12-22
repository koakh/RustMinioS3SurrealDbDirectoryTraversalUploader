use chrono::{DateTime, Utc};
use from_os_str::try_from_os_str;
use sha256::try_digest;
use std::{
  collections::HashMap,
  fmt::Display,
  fs::{self, Metadata, read_link},
  path::Path,
  time::SystemTime,
};
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

use from_os_str::Wrap;
use from_os_str::*;

use crate::{Args, Result};

/// stores directory paths, used to get node parent path <path, uuid>
type ParentPathHashMap = HashMap<String, Uuid>;

#[derive(Debug)]
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
#[derive(Debug)]
struct Node {
  uuid: Uuid,
  node_type: NodeType,
  name: String,
  path: String,
  // used to get real path from symlink
  canonical_path: Option<String>,
  size: u64,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
  accessed: DateTime<Utc>,
  sha256: Option<String>,
  parent_uuid: Uuid,
  notes: Option<String>,
}

/// check if dirEntry is hidden
fn is_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}

// https://stackoverflow.com/questions/64146345/how-do-i-convert-a-systemtime-to-iso-8601-in-rust
fn iso8601_to_string(st: &std::time::SystemTime) -> String {
  let dt: DateTime<Utc> = st.clone().into();
  format!("{}", dt.format("%+"))
  // formats like "2001-07-08T00:34:60.026490+09:30"
}

fn iso8601(st: &std::time::SystemTime) -> DateTime<Utc> {
  let dt: DateTime<Utc> = st.clone().into();
  dt
}

/// init walker traverse directory
pub fn process_dirs(args: &Args) -> Result<()> {
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
        let uuid = Uuid::new_v4();
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

        // first dir is always a dir, we assign it the first uuid
        match node_type {
          NodeType::Unknown => {}
          NodeType::Dir => {
            // insert a key only if it doesn't already exist
            parent_path_hash_map
              .entry(v.path().display().to_string())
              .or_insert(uuid);
          }
          NodeType::File => {}
          NodeType::Symlink => {
            canonical_path = Some(read_link(v.path()).unwrap().display().to_string())
          }
        }

        // always get path from hashmap, to use it with same uuid and path
        let mut current_parent_from_hash_path: String = String::default();
        let mut current_parent_from_hash_uuid: Uuid = Uuid::default();
        if let Some(v) = parent_path_hash_map.get_key_value(&input_path_parent) {
          current_parent_from_hash_path = (*v.0.clone()).to_string();
          current_parent_from_hash_uuid = *v.1;
        }
        // if !current_parent_from_hash_path.starts_with("/") {
        //   current_parent_from_hash_path = current_parent_from_hash_path.replace(&args.path, "/")
        // }

        let node = Node {
          uuid: Uuid::new_v4(),
          node_type,
          name,
          canonical_path,
          path: format!("/{current_parent_from_hash_path}"),
          size: metadata.len(),
          created:  iso8601(&metadata.created().unwrap()),
          modified: iso8601(&metadata.modified().unwrap()),
          accessed: iso8601(&metadata.accessed().unwrap()),
          sha256,
          parent_uuid: current_parent_from_hash_uuid,
          notes: None,
        };
        // exclude root node
        if nodes > 1 {
          println!("#{} path: {}", nodes, v.path().display());
          println!("\tname: {}, path: {}, node_type: {}, parent_uuid: {}\n", node.name, node.path, node.node_type, node.parent_uuid);
          println!(
            "{:#?}\n----------------------------------------------------------------------------------------------------------------------------------",
            node
          );
        }
      }
      Err(e) => println!("Error: {}", e),
    }
  }
  // debug final parent_path_hash_map
  // println!("parent_path_hash_map: {:#?}\n", parent_path_hash_map);
  Ok(())
}
