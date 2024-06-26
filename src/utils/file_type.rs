#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FileCategory {
    Unknown,
    Image,
    Video,
    Audio,
    Pdf,
    Archive,
    Document,
    Url,
    Other,
}

impl std::fmt::Display for FileCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "unknown"),
            Self::Image => write!(f, "image"),
            Self::Video => write!(f, "video"),
            Self::Audio => write!(f, "audio"),
            Self::Pdf => write!(f, "pdf"),
            Self::Archive => write!(f, "archive"),
            Self::Document => write!(f, "document"),
            Self::Url => write!(f, "url"),
            Self::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileType {
    pub file_category: FileCategory,
    pub kind_of_document: String,
    pub extension: Vec<String>,
    pub mime_type: Vec<String>,
    pub thumbnail: String,
}

impl FileType {
    fn new(file_category: FileCategory, kind_of_document: &str, extension: Vec<&str>, mime_type: Vec<&str>, bucket_name: &str, base_path: &str, thumbnail_extension: &str) -> Self {
        Self {
            file_category,
            kind_of_document: kind_of_document.into(),
            extension: extension.iter().map(|f| f.to_string()).collect(),
            mime_type: mime_type.iter().map(|f| f.to_string()).collect(),
            thumbnail: format!("{}/{}/{}.{}", bucket_name, base_path, extension[0], thumbnail_extension),
        }
    }
}

// get thumbnail
pub fn get_file_type(file_extension: &String, bucket_name: &str, base_path: &str, thumbnail_extension: &str) -> FileType {
    match file_extension.as_str() {
        // TODO: miss icon
        "pdf" => FileType::new(
            FileCategory::Pdf,
            "Portable Document Format",
            vec!["pdf"],
            vec!["application/pdf"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        // TODO: miss icon
        "wma" => FileType::new(
            FileCategory::Audio,
            "Windows Media Audio",
            vec!["wma"],
            vec!["audio/x-ms-wma"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        // TODO: miss icon
        "rtf" => FileType::new(
            FileCategory::Document,
            "Rich Text Format (RTF)",
            vec!["rtf"],
            vec!["application/rtf"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "3g2" => FileType::new(
            FileCategory::Video,
            "3GPP2 audio/video container",
            vec!["3g2"],
            vec!["video/3gpp2"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "3gp" => FileType::new(
            FileCategory::Video,
            "3GPP audio/video container",
            vec!["3gp"],
            vec!["video/3gpp"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "7z" => FileType::new(
            FileCategory::Archive,
            "7-zip archive",
            vec!["7z"],
            vec!["application/x-7z-compressed"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "aac" => FileType::new(FileCategory::Audio, "AAC audio", vec!["aac"], vec!["audio/aac"], bucket_name, base_path, thumbnail_extension),
        "abw" => FileType::new(
            FileCategory::Document,
            "AbiWord document",
            vec!["abw"],
            vec!["application/x-abiword"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "arc" => FileType::new(
            FileCategory::Archive,
            "Archive document (multiple files embedded)",
            vec!["arc"],
            vec!["application/x-freearc"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "avi" => FileType::new(
            FileCategory::Video,
            "AVI: Audio Video Interleave",
            vec!["avi"],
            vec!["video/x-msvideo", "video/x-avi", "video/avi", "video/divx", "video/msvideo", "video/vnd.divx"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "azw" => FileType::new(
            FileCategory::Document,
            "Amazon Kindle eBook format",
            vec!["azw"],
            vec!["application/vnd.amazon.ebook"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "bin" => FileType::new(
            FileCategory::Other,
            "Any kind of binary data",
            vec!["bin"],
            vec!["application/octet-stream"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "bmp" => FileType::new(
            FileCategory::Image,
            "Windows OS/2 Bitmap Graphics",
            vec!["bpm"],
            vec!["image/bmp"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "bz" => FileType::new(
            FileCategory::Archive,
            "BZip archive",
            vec!["bz"],
            vec!["application/x-bzip"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "bz2" => FileType::new(
            FileCategory::Archive,
            "BZip2 archive",
            vec!["bz2"],
            vec!["application/x-bzip2"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "csv" => FileType::new(
            FileCategory::Other,
            "Comma-separated values (CSV)",
            vec!["csv"],
            vec!["text/csv"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "doc" => FileType::new(
            FileCategory::Document,
            "Microsoft Word",
            vec!["doc"],
            vec!["application/msword"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "docx" => FileType::new(
            FileCategory::Document,
            "Microsoft Word (OpenXML)",
            vec!["docx"],
            vec!["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "epub" => FileType::new(
            FileCategory::Document,
            "Electronic publication (EPUB)",
            vec!["epub"],
            vec!["application/epub+zip"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "flv" => FileType::new(FileCategory::Video, "Flash video", vec!["flv"], vec!["video/x-flv"], bucket_name, base_path, thumbnail_extension),
        "gif" => FileType::new(
            FileCategory::Image,
            "Graphics Interchange Format (GIF)",
            vec!["giv"],
            vec!["image/gif"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "gz" => FileType::new(
            FileCategory::Archive,
            "Gzipped Tar File",
            vec!["gz"],
            vec!["application/x-compressed-tar"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "html" => FileType::new(
            FileCategory::Document,
            "HyperText Markup Language (HTML)",
            vec!["html"],
            vec!["text/html"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "jpg" => FileType::new(FileCategory::Image, "JPEG images", vec!["jpg"], vec!["image/jpeg"], bucket_name, base_path, thumbnail_extension),
        "jpeg" => FileType::new(FileCategory::Image, "JPEG images", vec!["jpeg"], vec!["image/jpeg"], bucket_name, base_path, thumbnail_extension),
        "mid" => FileType::new(
            FileCategory::Audio,
            "Musical Instrument Digital Interface (MIDI)",
            vec!["mid", "midi"],
            vec!["audio/midi audio/x-midi"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "mov" => FileType::new(
            FileCategory::Video,
            "Quicktime video",
            vec!["mov"],
            vec!["video/quicktime"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "mp2t" => FileType::new(
            FileCategory::Video,
            "MPEG transport stream",
            vec!["mp2t"],
            vec!["video/mp2t"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "mp3" => FileType::new(FileCategory::Audio, "MP3 audio", vec!["mp3"], vec!["audio/mpeg"], bucket_name, base_path, thumbnail_extension),
        "mp4" => FileType::new(FileCategory::Video, "Flash video", vec!["mp4"], vec!["video/mp4"], bucket_name, base_path, thumbnail_extension),
        "mpeg" => FileType::new(FileCategory::Video, "MPEG Video", vec!["mpeg"], vec!["video/mpeg"], bucket_name, base_path, thumbnail_extension),
        "odp" => FileType::new(
            FileCategory::Document,
            "OpenDocument presentation document",
            vec!["odp"],
            vec!["'application/vnd.oasis.opendocument.presentation'"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "ods" => FileType::new(
            FileCategory::Document,
            "OpenDocument spreadsheet document",
            vec!["ods"],
            vec!["application/vnd.oasis.opendocument.spreadsheet"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "odt" => FileType::new(
            FileCategory::Document,
            "OpenDocument text document",
            vec!["odt"],
            vec!["application/vnd.oasis.opendocument.text"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "ogg" => FileType::new(FileCategory::Audio, "OGG audio", vec!["ogg"], vec!["audio/ogg"], bucket_name, base_path, thumbnail_extension),
        "oga" => FileType::new(FileCategory::Audio, "OGG audio", vec!["ogg"], vec!["audio/ogg"], bucket_name, base_path, thumbnail_extension),
        "ogv" => FileType::new(FileCategory::Video, "OGG video", vec!["ogv"], vec!["video/ogg"], bucket_name, base_path, thumbnail_extension),
        "ogx" => FileType::new(FileCategory::Video, "OGG", vec!["ogx"], vec!["application/ogg"], bucket_name, base_path, thumbnail_extension),
        "opus" => FileType::new(FileCategory::Audio, "audio", vec!["opus"], vec!["audio/opus"], bucket_name, base_path, thumbnail_extension),
        "png" => FileType::new(
            FileCategory::Image,
            "Portable Network Graphics",
            vec!["png"],
            vec!["image/png"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "ppt" => FileType::new(
            FileCategory::Document,
            "Microsoft PowerPoint",
            vec!["ppt"],
            vec!["application/vnd.ms-powerpoint"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "cdr" => FileType::new(
            FileCategory::Document,
            "CorelDraw",
            vec!["dwg"],
            vec!["application/cdr"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "dwg" => FileType::new(
            FileCategory::Document,
            "AutoCad",
            vec!["dwg"],
            vec!["application/dwg"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "rar" => FileType::new(
            FileCategory::Archive,
            "RAR archive",
            vec!["rar"],
            vec!["application/vnd.rar"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "svg" => FileType::new(
            FileCategory::Image,
            "Scalable Vector Graphics (SVG)",
            vec!["svg"],
            vec!["image/svg+xml"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "tar" => FileType::new(
            FileCategory::Archive,
            "Tape Archive (TAR)",
            vec!["tar"],
            vec!["application/x-tar"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "tgz" => FileType::new(
            FileCategory::Archive,
            "Gzipped Tar File",
            vec!["tgz"],
            vec!["application/x-compressed-tar"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "tiff" => FileType::new(
            FileCategory::Image,
            "Tagged Image File Format (TIFF)",
            vec!["tiff"],
            vec!["image/tiff"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "tif" => FileType::new(
            FileCategory::Image,
            "Tagged Image File Format (TIFF)",
            vec!["tif"],
            vec!["image/tiff"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "ts" => FileType::new(
            FileCategory::Video,
            "MPEG transport stream",
            vec!["ts"],
            vec!["video/mp2t"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "txt" => FileType::new(
            FileCategory::Document,
            "Text (generally ASCII or ISO 8859-n)",
            vec!["txt"],
            vec!["text/plain"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "wav" => FileType::new(
            FileCategory::Audio,
            "Waveform Audio Format",
            vec!["wav"],
            vec!["audio/wav"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "weba" => FileType::new(FileCategory::Audio, "WEBM audio", vec!["weba"], vec!["audio/webm"], bucket_name, base_path, thumbnail_extension),
        "webm" => FileType::new(FileCategory::Video, "WEBM video", vec!["webm"], vec!["video/webm"], bucket_name, base_path, thumbnail_extension),
        "webp" => FileType::new(FileCategory::Image, "WEBP image", vec!["webp"], vec!["image/webp"], bucket_name, base_path, thumbnail_extension),
        "wmv" => FileType::new(
            FileCategory::Video,
            "Windows Media file with audio and/or video content",
            vec!["wmv"],
            vec!["video/x-ms-wmv"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "xhtml" => FileType::new(
            FileCategory::Document,
            "XHTML",
            vec!["xhtml"],
            vec!["application/xhtml+xml"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "xls" => FileType::new(
            FileCategory::Document,
            "Microsoft Excel",
            vec!["xls"],
            vec!["application/xhtml+xml", "application/vnd.ms-excel"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "xlsx" => FileType::new(
            FileCategory::Document,
            "Microsoft Excel (OpenXML)",
            vec!["xlsx"],
            vec!["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        "zip" => FileType::new(
            FileCategory::Archive,
            "ZIP archive",
            vec!["zip"],
            vec!["application/x-zip-compressed", "application/zip", "multipart/x-zip"],
            bucket_name,
            base_path,
            thumbnail_extension,
        ),
        _ => FileType::new(FileCategory::Unknown, "Unknown", vec!["unknown"], vec!["unknown/unknow"], bucket_name, base_path, thumbnail_extension),
    }
}
