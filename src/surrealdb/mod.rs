use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
  pub client: Surreal<Client>,
  pub name_space: String,
  pub db_name: String,
}

impl Database {
  pub async fn init() -> Result<Self, Error> {
    let url = std::env::var("SURREALDB_URL").expect("SURREALDB_URL must be set");
    let name_space = std::env::var("SURREALDB_NS").expect("SURREALDB_NS must be set");
    let db_name = std::env::var("SURREALDB_DB").expect("SURREALDB_DB must be set");
    let username = std::env::var("SURREALDB_USER").expect("SURREALDB_USER must be set");
    let password = std::env::var("SURREALDB_PASS").expect("SURREALDB_PASS must be set");
    let client = Surreal::new::<Ws>(url).await?;
    client
      .signin(Root {
        username: username.as_str(),
        password: password.as_str(),
      })
      .await?;
    client.use_ns(&name_space).use_db(&db_name).await.unwrap();

    Ok(Database { client, name_space, db_name })
  }
}
