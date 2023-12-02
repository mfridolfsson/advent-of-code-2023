use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let file_path = "week1.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
