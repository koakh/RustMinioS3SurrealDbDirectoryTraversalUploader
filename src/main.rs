use clap::Parser;
use std::error::Error;
use std::result;
use walker::process_dirs;

mod db;
mod walker;

// declare a generic result tye
pub type Result<T> = result::Result<T, Box<dyn Error>>;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    // process directories
    match process_dirs(&args) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("Error: {}", e);
            Ok(())
        }
    }
}
