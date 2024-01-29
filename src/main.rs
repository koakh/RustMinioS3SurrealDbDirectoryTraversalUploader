use clap::Parser;
use dotenv::dotenv;
use std::error::Error;
use std::result;
use surrealdb::Database;
use walker::process_dirs;

mod surrealdb;
mod walker;
// mod sqlx;

// declare a generic result tye
pub type Result<T> = result::Result<T, Box<dyn Error>>;

/// simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// name of the person to greet
  #[arg(short, long)]
  path: String,
  /// number of times to greet
  #[arg(short, long, default_value_t = 1)]
  count: u8,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let args = Args::parse();
  // read .env
  dotenv().ok();

  // init datastore
  let db = Database::init()
    .await
    .expect("error connecting to database");

  // process directories
  match process_dirs(&args, &db).await {
    Ok(_) => Ok(()),
    Err(e) => {
      eprint!("Error: {}", e);
      Ok(())
    }
  }
}
