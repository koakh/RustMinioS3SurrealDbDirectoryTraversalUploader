use from_os_str::try_from_os_str;
use sha256::try_digest;
use std::{
  collections::HashMap,
  ffi::OsStr,
  fmt::Display,
  fs::{self, Metadata},
  ops::Deref,
  path::Path,
  time::SystemTime,
};
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

use from_os_str::Wrap;
use from_os_str::*;

use crate::{Args, Result};

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
  size: u64,
  created: SystemTime,
  modified: SystemTime,
  accessed: SystemTime,
  sha256: Option<String>,
  parent_uuid: Uuid,
  // parent_node: Option<DirEntry>,
  // parent_path: String,
  notes: Option<String>,
}

/// stores directory paths, used to get node parent path <path, uuid>
type ParentPathHashMap = HashMap<Uuid, String>;

// https://doc.rust-lang.org/std/collections/struct.HashMap.html
// #[derive(Hash, Eq, PartialEq, Debug)]
// struct ParentPath {
//   key: String,
//   value: Uuid,
// }

// impl ParentPath {
//   fn new(key: String, value: Uuid) -> Self {
//     Self { key, value }
//   }
// }

fn is_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}

// TODO: know if is dir, file or link
// std::fs::read_link seems what you want.

pub fn process_dirs(args: &Args) -> Result<()> {
  // println!("traverse paths {}", &args.path);
  let mut nodes = 0;
  let mut parent_path_hash_map = ParentPathHashMap::new();
  let walker = WalkDir::new(&args.path)
    .follow_links(true)
    .sort_by_file_name()
    .into_iter();
  // TODO:
  for entry in walker.filter_entry(|e| !is_hidden(e)) {
    nodes += 1;
    // let mut uuid = Uuid::new_v4();
    // must initialize parent_uuid from first uuid
    let mut parent_uuid = Uuid::new_v4();
    // let mut parent_path = String::from("root");
    match entry {
      Ok(v) => {
        // let path = v.path().display().to_string().replace(&args.path, "");
        // return if is source dir
        // if name.eq("") {
        //   break;
        // };
        let uuid = Uuid::new_v4();
        let metadata = fs::symlink_metadata(v.path())?;
        // let metadata: Metadata = fs::metadata(v.path())?;
        let node_type: NodeType = metadata.clone().into();
        let input_path = Path::new(v.path());
        // the name from input path
        // let name = input_path.file_stem().unwrap().to_owned().to_owned().to_str().unwrap().to_string();
        // let name = v.file_name();
        // let os_str = OsStr::new("123");
        let os_str = v.file_name();
        let name = try_from_os_str!(os_str as &Path)
          .unwrap()
          .display()
          .to_string();
        // the path without filename from input path
        let input_path_parent = input_path.parent().unwrap().display().to_string();
        let sha256 = match try_digest(input_path) {
          Ok(v) => Some(v),
          Err(_) => None,
        };

        // first dir is always a dir, we assin it the first uuid
        match node_type {
          NodeType::Unknown => {}
          NodeType::Dir => {
            // insert a key only if it doesn't already exist
            parent_path_hash_map
              .entry(uuid)
              .or_insert(input_path_parent.clone());
          }
          NodeType::File => {}
          NodeType::Symlink => {}
        }

        // always get path from hashmap, to use it with same uuid and path
        // let current_parent_from_hash_map: (&String, &Uuid);
        let mut current_parent_from_hash_uuid: Uuid = Uuid::default();
        let mut current_parent_from_hash_path: String = String::default();
        if let Some(v) = parent_path_hash_map.get_key_value(&uuid) {
          current_parent_from_hash_uuid = *v.0;
          current_parent_from_hash_path = (*v.1.clone()).to_string();
        }

        // if let Some(i) = input{
        //     passInput = PreUpdateInput{channel: i.channel.clone()};
        //   };
        let node = Node {
          uuid: Uuid::new_v4(),
          node_type,
          name,
          path: current_parent_from_hash_path,
          size: metadata.len(),
          created: metadata.created().unwrap(),
          modified: metadata.modified().unwrap(),
          accessed: metadata.accessed().unwrap(),
          sha256,
          parent_uuid: current_parent_from_hash_uuid,
          // parent_path,
          notes: None,
        };
        // exclude root node
        if nodes > 0 {
          println!("path: {}", v.path().display());
          println!("\tname: {}, path: {}, node_type: {}, parent_uuid: {}\n", node.name, node.path, node.node_type, node.parent_uuid);
          // println!("\t\tinput_path_parent: {}", input_path_parent);
        }
        // println!("{:#?}", node);
      }
      Err(e) => println!("Error: {}", e),
    }
  }
  Ok(())
}
