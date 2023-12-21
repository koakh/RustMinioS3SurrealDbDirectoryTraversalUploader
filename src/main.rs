use clap::Parser;
use walker::process_dirs;
use std::result;
use std::error::Error;

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

fn main() {
    let args = Args::parse();

    let _ = process_dirs(&args);
}
