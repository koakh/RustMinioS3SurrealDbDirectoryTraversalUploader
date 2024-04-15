use from_os_str::try_from_os_str;
use from_os_str::Wrap;
use from_os_str::*;
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

use crate::utils::file_type::FileCategory;
use crate::utils::file_type::FileType;
use crate::utils::shell_command::execute_command_shortcut;
use crate::{
    app::{
        ARGS_PROCCESS_S3_UPLOAD, ARGS_PROCCESS_THUMBNAILS, S3_BUCKET_DOWNLOADS_PATH, S3_BUCKET_THUMBNAIL_PATH, STATIC_FILES_DIRECTORY_ICON_PATH, STATIC_FILES_IMAGES_MIME_TYPE_BASE_PATH,
        STATIC_FILES_IMAGES_MIME_TYPE_EXT, STORAGE_NODE_TABLE, THUMBNAIL_FORMAT, THUMBNAIL_SIZES, THUMBNAIL_TEMPORARY_PATH,
    },
    minio::Client,
    surrealdb::Database,
    utils::{datetime::st2sdt, file_type::get_file_type},
    Args, Result,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct ParentPathProp {
    thing: Thing,
    ancestors: Vec<Thing>,
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageNode {
    id: Thing,
    node_type: NodeType,
    name: String,
    file_name: Option<String>,
    file_extension: Option<String>,
    file_duplicated: Option<bool>,
    path: String,
    full_path: String,
    // used to get real path from symlink
    canonical_path: Option<String>,
    size: u64,
    created: SdbDatetime,
    modified: SdbDatetime,
    accessed: SdbDatetime,
    sha256: Option<String>,
    s3_url: Option<String>,
    s3_thumbnail: Option<String>,
    parent_id: Thing,
    ancestors: Vec<Thing>,
    notes: Option<String>,
}

// using raw query
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
            .create(Resource::from(STORAGE_NODE_TABLE))
            .content(self)
            .await?;
        Ok(())
    }

    async fn _exist(&self, db: &Database) -> Result<bool> {
        let exists: Option<Self> = db
            .client
            .select((STORAGE_NODE_TABLE, &self.id.id.to_string()))
            .await?;
        match exists {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    // associative method
    async fn record_already_exists(db: &Database, filter: String) -> Result<bool> {
        let sql = format!("SELECT * FROM type::table($table) WHERE {}", filter);
        let mut result = db
            .client
            .query(sql)
            .bind(("table", STORAGE_NODE_TABLE))
            .await?
            // check if have errors
            .check()?;
        // get the first result from the first query
        let exists: Option<Self> = result.take(0)?;
        match exists {
            Some(_) => Ok(true),
            None => Ok(false),
        }
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
pub async fn process_dirs(args: &Args, db: &Database, s3_client: &Client, bucket_name: &String) -> Result<()> {
    // println!("traverse paths {}", &args.path);
    let mut storage_nodes = 0;
    let mut parent_path_hash_map = ParentPathHashMap::new();
    let walker = WalkDir::new(&args.path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        storage_nodes += 1;
        // in every iter always set it to false, and when we check if exists we override this value, and in the end on save node, if is true we use it to skip save
        let mut record_already_exists = false;
        match entry {
            Ok(v) => {
                println!("#{} path: {}", storage_nodes, v.path().display());
                let node_id;
                // if first node, use a fixed root id, will be storage_node:root
                if parent_path_hash_map.len() == 0 {
                    node_id = DdbId::String("root".into())
                } else {
                    node_id = DdbId::rand();
                }
                // set current storage node thing, based on node_id defined above
                let id: Thing = Thing {
                    tb: STORAGE_NODE_TABLE.into(),
                    id: node_id.clone(),
                };
                let metadata = fs::symlink_metadata(v.path())?;
                let node_type: NodeType = metadata.clone().into();
                let input_path = Path::new(v.path());
                let os_str = v.file_name();
                // set filename, ot none for dir
                let mut file_name_option: Option<String> = None;
                // always havea working file_name to work on loop
                let file_name = try_from_os_str!(os_str as &Path)
                    .unwrap()
                    .display()
                    .to_string();
                // override file_name_option, the one that will be used in record persistence
                if node_type == NodeType::File {
                    file_name_option = Some(file_name.clone());
                }
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

                // defined defaults
                // always get path from hashmap, to use it with same id and path
                let mut current_parent_from_hash_path: String = String::default();
                // let mut current_parent_thing_from_hashmap_pathkey: Uuid = Uuid::default();
                let mut current_parent_thing_from_hashmap_pathkey: Thing = Thing {
                    tb: STORAGE_NODE_TABLE.into(),
                    id: DdbId::rand(),
                };

                // default current_ancestors
                let mut current_parent_ancestors_from_hashmap_pathkey = Vec::<Thing>::new();
                // try get it from hasmap, if exists override defaults defined above
                if let Some(v) = parent_path_hash_map.get_key_value(&input_path_parent) {
                    current_parent_from_hash_path = (*v.0.clone()).to_string();
                    current_parent_thing_from_hashmap_pathkey = v.1.thing.clone();
                    // clone parent ancestors
                    current_parent_ancestors_from_hashmap_pathkey = v.1.ancestors.clone();
                    // now we push current parent into current_ancestors
                    current_parent_ancestors_from_hashmap_pathkey.push(current_parent_thing_from_hashmap_pathkey.clone());
                }

                // must be after defining current_ancestors above, above is where we have the current_parent_thing_from_hashmap_pathkey

                // first iter is always a dir, we assign it the first id
                match node_type {
                    NodeType::Unknown => {}
                    NodeType::Dir => {
                        // insert a key only if it doesn't already exist
                        let key = v.path().display().to_string();
                        parent_path_hash_map.entry(key).or_insert(ParentPathProp {
                            thing: id.clone(),
                            ancestors: current_parent_ancestors_from_hashmap_pathkey.clone(),
                        });
                    }
                    NodeType::File => {}
                    NodeType::Symlink => canonical_path = Some(read_link(v.path()).unwrap().display().to_string()),
                }

                // remove root (source path) from final path, and assign / to it
                let path;
                let replace = format!("{}", &args.path);
                if !current_parent_from_hash_path.eq(&args.path) {
                    // println!("replace: {}, current_parent_from_hash_path: {}", replace, current_parent_from_hash_path);
                    path = current_parent_from_hash_path.replace(&replace, "");
                } else {
                    path = "/".into();
                }

                // clone name into filename before
                // get name without extension
                let name = match Path::new(&file_name).file_stem() {
                    Some(v) => v.to_string_lossy().to_string(),
                    None => file_name.clone(),
                };

                let full_path = format!("{}/{}", &current_parent_from_hash_path, &file_name).replace(&replace, "");
                println!("  full_path: {}", full_path);
                let mut file_extension = None;
                let mut thumbnail = None;
                let mut file_duplicated = None;

                // define s3 url
                let mut s3_url: Option<String> = None;
                if node_type == NodeType::File {
                    // check if file with sha256 already exists and skip it
                    if let Some(t) = &sha256 {
                        // using sha256 exists filter
                        let exists_result = StorageNode::record_already_exists(&db, format!("sha256 = '{}';", &t)).await;
                        if let Ok(exists) = exists_result {
                            if exists {
                                // this will be setted only in first traversal run, next runs the file fullPath alreasdy exists and even this will be setted to true here
                                // the record will not be saved /persisted because of next record_already_exists will br true, seel bellow on next exist
                                file_duplicated = Some(true);
                            }
                        }
                        // using fullPath exists filter
                        let exists_result = StorageNode::record_already_exists(&db, format!("fullPath = '{}';", &full_path)).await;
                        if let Ok(exists) = exists_result {
                            // using sha256 exists filter
                            // if exists && SKIP_EXISTING_FILES_WITH_SAME_SHA256 {
                            // using fullPath exists filter
                            if exists {
                                // using sha256 exists filter
                                // println!("  skip file '{}' with sha256: '{}' already exists", &file_name, &t);
                                // using fullPath exists filter
                                println!("  skip file '{}' with fullPath: '{}' already exists", &file_name, &full_path);
                                // override default from loop
                                record_already_exists = true;
                            } else {
                                if ARGS_PROCCESS_S3_UPLOAD {
                                    let upload_file = format!("{}/{}", &current_parent_from_hash_path, &file_name);
                                    // always remove root args path from key, and start slash
                                    let key = format!("{}/{}", S3_BUCKET_DOWNLOADS_PATH, &upload_file.replace(&args.path, "")[1..]);
                                    // key must be equal to file path without root path in this case is upload_file ex '/root.file'
                                    println!("  uploading: {}, key: {}", &upload_file, &key);
                                    // always remove base endpoint from
                                    let (_, s3_bucket_name_key) = s3_client.put_object_from_file(&upload_file, &key).await;
                                    // override default
                                    s3_url = Some(s3_bucket_name_key);
                                } // if ARGS_PROCCESS_S3_UPLOAD {

                                // override None file extension
                                file_extension = match Path::new(&file_name).extension() {
                                    Some(v) => Some(v.to_string_lossy().to_string()),
                                    None => Some("unk".into()),
                                };

                                if ARGS_PROCCESS_THUMBNAILS {
                                    // get thumbnail
                                    let mut s3_thumbnail: Option<FileType> = None;
                                    if node_type == NodeType::File {
                                        s3_thumbnail = Some(get_file_type(
                                            // we can use unwrap here, above we assure that is allways a extension in anyy case
                                            &file_extension.clone().unwrap(),
                                            bucket_name.as_str(),
                                            STATIC_FILES_IMAGES_MIME_TYPE_BASE_PATH,
                                            STATIC_FILES_IMAGES_MIME_TYPE_EXT,
                                        ));
                                        thumbnail = match s3_thumbnail.clone() {
                                            Some(v) => Some(v.thumbnail),
                                            None => None,
                                        };
                                    } else
                                    /*if node_type == NodeType::Dir*/
                                    {
                                        thumbnail = Some(format!(
                                            "{}/{}/{}.{}",
                                            bucket_name.as_str(),
                                            S3_BUCKET_THUMBNAIL_PATH,
                                            STATIC_FILES_DIRECTORY_ICON_PATH,
                                            STATIC_FILES_IMAGES_MIME_TYPE_EXT
                                        ));
                                    }

                                    // generate thumbnails
                                    match s3_thumbnail {
                                        Some(v) => match &v.file_category {
                                            FileCategory::Image => {
                                                // println!("generate thumbnail for {} - {}", v.file_category, input_path.display());
                                                for index in THUMBNAIL_SIZES {
                                                    // base commands with and without implicit file format, in this case png
                                                    // convert "$file" -resize 100x100^ -gravity center -extent 100x100 "${file%.*}_thumbnail.${file##*.}"
                                                    // convert "$file" -resize 100x100^ -gravity center -extent 100x100 PNG:"${file%.*}_thumbnail.png"
                                                    let command = format!(
                                                        "file=\"{0}\" && cd \"{1}\" && convert \"$file\" -resize {2}^ -gravity center -extent {2} {3}:\"{5}/${{file%.*}}_{2}.{4}\"",
                                                        file_name,
                                                        format!("{}{}", &args.path, path),
                                                        index,
                                                        THUMBNAIL_FORMAT,
                                                        THUMBNAIL_FORMAT.to_lowercase(),
                                                        THUMBNAIL_TEMPORARY_PATH
                                                    );
                                                    // println!("command: {}", command);
                                                    match execute_command_shortcut(&command) {
                                                        Ok(_) => {
                                                            // upload to s3 storage and remove tem file
                                                            let upload_file_name = format!("{}_{}.{}", name, index, THUMBNAIL_FORMAT.to_lowercase());
                                                            let upload_file_path = format!("{}/{}", THUMBNAIL_TEMPORARY_PATH, upload_file_name);
                                                            // check if thumbnail file exists, this is not required, if reach here thumbnail is generated, but is a prevention
                                                            if let Ok(metadata) = fs::metadata(&upload_file_path) {
                                                                if metadata.is_file() {
                                                                    // always remove root args path from key, and start slash
                                                                    let key = format!("{}/{}/{}", S3_BUCKET_THUMBNAIL_PATH, &path.replace(&args.path, "")[1..], upload_file_name);
                                                                    // key must be equal to file path without root path in this case is upload_file ex '/root.file'
                                                                    println!("  uploading thumbnail: {}, key: {}", &upload_file_path, &key);
                                                                    // always remove base endpoint from
                                                                    let (_, s3_bucket_name_key) = s3_client
                                                                        .put_object_from_file(&upload_file_path, &key)
                                                                        .await;
                                                                    // override default mimeType generated thumbnail, with image thumbnail
                                                                    thumbnail = Some(s3_bucket_name_key);
                                                                    // remove temp file
                                                                    fs::remove_file(&upload_file_path).expect(&format!("{} file delete failed", &upload_file_path));
                                                                } else {
                                                                    println!("skipped upload thumbnail file '{}', file exists, but it's not a regular file.", &upload_file_path);
                                                                }
                                                            } else {
                                                                println!("skipped upload thumbnail file '{}', file does not exist.", &upload_file_path);
                                                            }
                                                        }
                                                        Err(e) => {
                                                            eprintln!("error: {e}");
                                                        }
                                                    }
                                                }
                                            }
                                            FileCategory::Video => {
                                                // println!("generate thumbnail for {} - {}", v.file_category, input_path.display());
                                            }
                                            _ => {}
                                        },
                                        None => {}
                                    };
                                } // if ARGS_PROCCESS_THUMBNAILS {
                            } // if sha256 exits
                        } // if sha256 exits
                    } // if sha256 exits
                } // if node_type == NodeType::File {

                if node_type == NodeType::Dir {
                    let exists_result = StorageNode::record_already_exists(&db, format!("fullPath = '{}';", &full_path)).await;
                    if let Ok(exists) = exists_result {
                        if exists {
                            println!("  skip dir '{}' already exists", &name);
                            // override default from loop
                            record_already_exists = true;
                        }
                    }
                }

                // create storageNode
                let node = StorageNode {
                    id,
                    node_type: node_type.clone(),
                    name,
                    file_name: file_name_option,
                    file_extension,
                    file_duplicated,
                    path: path.clone(),
                    full_path: full_path.clone(),
                    canonical_path,
                    size: metadata.len(),
                    created: st2sdt(&metadata.created().unwrap()),
                    modified: st2sdt(&metadata.modified().unwrap()),
                    accessed: st2sdt(&metadata.accessed().unwrap()),
                    sha256,
                    s3_url: s3_url.clone(),
                    s3_thumbnail: thumbnail,
                    parent_id: current_parent_thing_from_hashmap_pathkey,
                    ancestors: current_parent_ancestors_from_hashmap_pathkey,
                    notes: None,
                };

                // save storage_node, only if record is not already persisted
                if !record_already_exists {
                    match node.save(&db).await {
                        Ok(_) => {
                            println!(
                                "  storage node saved: node.type: {}, node_id: {}, node.id:tb:node:id:id: {}:{}",
                                &node.node_type, &node_id, &node.id.tb, &node.id.id
                            );
                        }
                        Err(e) => eprintln!("error saving node: {:#?}", e),
                    };
                }

                // println!(
                //     "storage node props name: {}, path: {}, node_type: {}, id: {}:{}, parent_id: {}\n",
                //     &node.name, &node.path, &node.node_type, &node.id.tb, &node.id.id, &node.parent_id
                // );
                print!("\n");
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    // debug final parent_path_hash_map
    // println!("parent_path_hash_map: {:#?}\n", parent_path_hash_map);
    // parent_path_hash_map
    //   .into_iter()
    //   .for_each(|(k, v)| println!("k: {} -> v: {}, ancestors: {:?}", k, v.thing, v.ancestors));

    Ok(())
}
