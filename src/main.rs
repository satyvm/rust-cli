use std::env;
use std::path;

struct Cli {
    pattern: String,
    path: path::PathBuf,
}

fn main() {
    let pattern = env::args().nth(1).expect("No Pattern Given");
    let path = env::args().nth(2).expect("No Path Given");

    println!("pattern:{:?}, path:{:?}", pattern, path)
}

