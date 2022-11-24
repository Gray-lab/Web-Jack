use std::{env, fs};

mod parser;
mod config;
use crate::{parser::parse, config::Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let config = Config::build(&args).expect("this could break in many ways");

    dbg!(&config);
    
    for file_path in config.file_paths {
        let contents = fs::read_to_string(file_path).expect("should be readable");
        let tokens = parse(&contents);
        dbg!(&tokens);
    }
}

