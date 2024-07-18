#![allow(unused_imports)]

/// first approach

use std::env;
use std::path::PathBuf;


// struct Cli {
//     pattern: String,
//     path: PathBuf,
// }

// fn main() {
//     let pattern = env::args().nth(1).expect("No Pattern Given");
//     let path = env::args().nth(2).expect("No Path Given");
//
//     println!("pattern:{:?}, path:{:?}", pattern, path)
// }

/// second approach

use clap::{Error, Parser};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use clap::error::ErrorKind;

#[derive(Parser)]

struct Cli{
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    // println!("pattern:{:?}, path:{:?}", args.pattern, args.path);

    // reading file without buffer (not optimized for large files)
    let file_content = fs::read_to_string(&args.path);
    let content = match file_content {
        Ok(text) => {
            // println!("File Content: {}", text);
            text
        }
        Err(error) => {
            // println!("Error: {}", error);
            panic!("Error: {}", error)
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }

    /// buffer implementation for reading the file

    // let buf_content = File::open(&args.path);
    // let mut reader = BufReader::new(buf_content);
    // for line in reader.buffer() {
    //     println!("{}", line);
    // }

}

