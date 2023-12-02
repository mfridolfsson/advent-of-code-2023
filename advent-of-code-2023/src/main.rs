
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;

fn main() {
    println!("Hello, world!");

    let file_path = "week1.txt";

    println!("In file {}", file_path);

    //let contents = fs::read_to_string(file_path)
    //    .expect("Should have been able to read the file");

    let lines = read_lines(file_path);
    
    let mut output = 0;

    for line in lines {
        let re = Regex::new(r"[^0-9.]").unwrap();
        let this_line = line.as_str();
        let numbers_only = re.replace_all(this_line ,"");
        let numbers_only_vec: Vec<char> = numbers_only.chars().collect();
        let len_numbers_only_vec = numbers_only_vec.len();
        let len_numbers_only_vec_minus_1 = len_numbers_only_vec-1;
        println!("line: {}", numbers_only);
        println!("len: {:?}", len_numbers_only_vec);
        let first_int = numbers_only_vec[0];
        println!("first_int: {}", first_int);
        let mut last_int;
        if len_numbers_only_vec > 1 {
            last_int = numbers_only_vec[len_numbers_only_vec_minus_1];
        } else {
            last_int = first_int;
        }
        println!("last_int: {}", last_int);
        let mut int_str = String::from("");
        int_str.push(first_int);
        int_str.push(last_int);
        println!("int: {}", int_str);
        let int_int = int_str.parse::<i32>().unwrap();
        println!("as int: {}", int_int);
        output = output + int_int;
        println!("running total: {}", output);
        //println!("{}", numbers_only);
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