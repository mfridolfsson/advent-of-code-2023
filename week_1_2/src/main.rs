
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;

fn main() {
    println!("Begin:");

    let file_path = "week1.txt";

    println!("In file {}", file_path);

    let lines = read_lines(file_path);
    
    let mut output = 0;

    for line in lines {
        let re = Regex::new(r"[^0-9.]").unwrap();
        let this_line = line.as_str();

        println!("line: {}", this_line);
        let numbers_only = re.replace_all(this_line ,"");
    }

    
    
}

// borroewed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}