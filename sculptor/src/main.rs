use std::path::PathBuf;

use clap::Parser;

mod err;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
    language: String,
}

fn main() {
    let args = Args::parse();
}
