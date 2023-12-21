use chrono::{DateTime, Utc};
use sha256::{digest, try_digest};
use std::{
  fmt::Display,
  fs::{self, Metadata},
  path::Path,
  time::SystemTime,
};
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

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

#[derive(Debug)]
struct Node {
  uuid: Uuid,
  node_type: NodeType,
  name: String,
  size: u32,
  created: SystemTime,
  modified: SystemTime,
  accessed: SystemTime,
  sha256: Option<String>,
  parent_uuid: Option<Uuid>,
  parent_path: Option<String>,
  notes: Option<String>,
}

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
  println!("traverse paths {}!", &args.path);
  let walker = WalkDir::new(&args.path)
    .follow_links(true)
    .sort_by_file_name()
    .into_iter();
  // TODO:
  let parent: DirEntry;
  for entry in walker.filter_entry(|e| !is_hidden(e)) {
    match entry {
      Ok(v) => {
        let name = v.path().display().to_string().replace(&args.path, "");
        // return if is source dir
        // if name.eq("") {
        //   break;
        // };

        // let metadata: Metadata = fs::metadata(v.path())?;
        let metadata = fs::symlink_metadata(v.path())?;
        // let node_type: NodeType = metadata.clone().into();

        // if let Some(i) = input{
        //     passInput = PreUpdateInput{channel: i.channel.clone()};
        //   };

        let input = Path::new(v.path());
        let sha256 = match try_digest(input) {
          Ok(v) => Some(v),
          Err(_) => None,
        };
        let node = Node {
          uuid: Uuid::new_v4(),
          node_type: metadata.clone().into(),
          name,
          size: 1,
          created: metadata.created().unwrap(),
          modified: metadata.modified().unwrap(),
          accessed: metadata.accessed().unwrap(),
          sha256,
          parent_uuid: None,
          parent_path: None,
          notes: None,
        };
        // println!("{}, node_type: {}, size: {}", v.path().display(), node_type, metadata.len());
        println!("{:#?}", node.name);
      }
      Err(e) => println!("Error: {}", e),
    }
  }
  Ok(())
}
