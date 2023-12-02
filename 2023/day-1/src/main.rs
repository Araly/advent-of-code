use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("input file: {file_path}");

    let contents = fs::read_to_string(file_path).expect("should read the file");

    println!("contents of input file:\n{contents}")
}
