use tokio::io::AsyncReadExt;

/// s3 client wrapper to expose semantic upload operations
#[derive(Debug, Clone)]
pub struct Client {
  s3: aws_sdk_s3::Client,
  bucket_name: String,
}

impl Client {
  /// construct s3 client wrapper.
  pub fn new() -> Self {
    let endpoint_url = std::env::var("S3_HOST").unwrap();
    let region = std::env::var("S3_REGION").unwrap();
    let key_id = std::env::var("S3_ACCESS_KEY_ID").unwrap();
    let secret_key = std::env::var("S3_SECRET_ACCESS_KEY").unwrap();
    let bucket_name = std::env::var("S3_BUCKET_NAME").unwrap();

    let credentials = aws_sdk_s3::config::Credentials::new(key_id, secret_key, None, None, "loaded-from-custom-env");
    let s3_config = aws_sdk_s3::config::Builder::new()
      .endpoint_url(endpoint_url)
      .credentials_provider(credentials)
      .region(aws_sdk_s3::config::Region::new(region))
      // apply bucketName as path param instead of pre-domain
      .force_path_style(true)
      .build();
    let client = aws_sdk_s3::Client::from_conf(s3_config);
    Self { s3: client, bucket_name }
  }

  fn url(&self, key: &str) -> String {
    format!("{}/{key}", std::env::var("S3_HOST").unwrap(),)
  }

  /// real upload of file to S3
  pub async fn put_object_from_file(&self, local_path: &str, key: &str) -> (String, String) {
    let mut file = tokio::fs::File::open(local_path).await.unwrap();

    let size_estimate = file
      .metadata()
      .await
      .map(|md| md.len())
      .unwrap_or(1024)
      .try_into()
      .expect("file too big");

    let mut contents = Vec::with_capacity(size_estimate);
    file.read_to_end(&mut contents).await.unwrap();

    let _res = self
      .s3
      .put_object()
      .bucket(&self.bucket_name)
      .key(key)
      .body(aws_sdk_s3::primitives::ByteStream::from(contents))
      .send()
      .await
      .expect("Failed to put object");

    // full url, with enpoint, bucket name, and key 
    // ex http://192.168.1.52:9000/default-bucket/root/root.file
    let s3_url = self.url(format!("{}/{}", &self.bucket_name, key).as_str());
    // s3 bucket name and key
    // ex /default-bucket/root/root.file
    let s3_bucket_name_key = format!("{}/{}", &self.bucket_name, key);

    (s3_url, s3_bucket_name_key)
  }

  /// attempts to delete object from S3. Returns true if successful.
  pub async fn _delete_file(&self, key: &str) -> bool {
    self
      .s3
      .delete_object()
      .bucket(&self.bucket_name)
      .key(key)
      .send()
      .await
      .is_ok()
  }
}
