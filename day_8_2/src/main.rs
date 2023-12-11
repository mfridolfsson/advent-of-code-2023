
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;
use std::collections::HashMap;

fn main() {
    println!("Begin:");


    let mut output = 0;

    let file_path = "input.txt";

    println!("In file {}", file_path);

    let mut lines: Vec<String> = read_lines(file_path);
    lines.retain(|x| x != "");

    let mut directions:String = String::new();

    let mut directions_map: HashMap<String, Vec<String>> = HashMap::new();

    for (index, this_line) in lines.iter().enumerate() {

        if index == 0 {
            directions = this_line.to_string();
            println!("directions: {:?}", directions);

        }
        else {
            println!("raw data: {:?}", this_line);
            let id_vs_data_vec:Vec<String> = this_line.split("=").map(|s| s.to_string()).collect();
            let id_string = id_vs_data_vec[0].as_str();
            let data_string = id_vs_data_vec[1].as_str();

            let re = Regex::new(r"[ ]").unwrap();
            let id_string = re.replace_all(id_string ,"");

            let re = Regex::new(r"[ ()]").unwrap();
            let data_string = re.replace_all(data_string ,"");
            let data_vec:Vec<String> = data_string.split(",").map(|s| s.to_string()).collect();

            println!("id_string: {:?}", id_string);
            println!("data_string: {:?}", data_vec);
            directions_map.insert(
                id_string.to_string(),
                data_vec,
            );    
        }
        
    }


    println!("directions: {:?}", directions);
    println!("map: {:?}", directions_map);

    

    for line in lines {

        let char_vec: Vec<char> = line.chars().collect();
        if char_vec[2] == 'A' {

            let mut this_location:String = &line[..4];

            //make moves
            while char_vec[2] != 'Z' {
                for c in directions.chars() { 
                    let left_or_right = if c == 'L' {0} else {1};
    
                    println!("location: {:?}", this_location);
                    println!("direction: {:?}", c);
                    
                    let mut this_direction: Vec<String> = Vec::new();
                    match directions_map.get(&this_location) {
                        Some(&ref direction) => this_direction = direction.to_vec(),
                        _  => println!("No loction found!"),
                    }
                
                    println!("direction: {:?}", this_direction[left_or_right]);
                    this_location = this_direction[left_or_right].to_string();
    
                    output = output + 1;
                    if this_location == "ZZZ".to_string(){break;}
                }
            }    

    }

    }

    println!("SOLUTION:{:?} ", output);

    
}


// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}