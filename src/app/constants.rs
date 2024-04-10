pub const STORAGE_NODE_TABLE: &str = "storage_node";
pub const S3_BUCKET_DOWNLOADS_PATH: &str = "downloads";
pub const S3_BUCKET_THUMBNAIL_PATH: &str = "thumbnails";
pub const STATIC_FILES_IMAGES_MIME_TYPE_BASE_PATH: &str = "images/mime-type";
pub const STATIC_FILES_DIRECTORY_ICON_PATH: &str = "images/other/folder";
pub const STATIC_FILES_IMAGES_MIME_TYPE_EXT: &str = "svg";
pub const THUMBNAIL_TEMPORARY_PATH: &str = "/tmp";
pub const THUMBNAIL_SIZES: [&str; 2] = ["200x200", "400x400"];
pub const THUMBNAIL_FORMAT: &str = "PNG";
// remove or use in args
pub const ARGS_PROCCESS_THUMBNAILS: bool = true;
pub const ARGS_PROCCESS_S3_UPLOAD: bool = true;