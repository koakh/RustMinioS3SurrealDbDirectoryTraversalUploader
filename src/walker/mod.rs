use std::fmt::Error;

use crate::{Args, Result};
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// TODO: know if is dir, file or link

pub fn process_dirs(args: &Args) -> Result<()> {
    println!("traverse paths {}!", &args.path);

    let walker = WalkDir::new(&args.path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(v) => {
                println!("{}", v.path().display());
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}
