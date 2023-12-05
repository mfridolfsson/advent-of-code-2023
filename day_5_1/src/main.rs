
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;
use std::collections::HashMap;

fn main() {
    println!("Begin:");

    let file_path = "input.txt";

    println!("In file {}", file_path);

    //let contents = fs::read_to_string(file_path)
    //    .expect("Should have been able to read the file");

    let mut output:i32 = 0;


    let mut lines = read_lines(file_path);
    println!("{:?}", lines);

    //remove blanks
    lines.retain(|x| x != "");

    //get seed ids into a vector
    let seeds_string = lines[0].as_str();
    let re = Regex::new(r"[^0-9 ]").unwrap();
    let just_seeds_values = re.replace_all(seeds_string ,"");
    let mut seeds:Vec<String> = just_seeds_values.split(" ").map(|s| s.to_string()).collect();
    seeds.retain(|x| x != "");
    println!("seeds: {:?}", seeds);

    lines.remove(0);

    //let mut mappings_hash_map = HashMap::new();
    
    let mut this_map_name = String::new();
    let mut this_map = String::new();

    for seed in seeds {

        let mut soil_destimation = 0;
        let mut fertiliser_destimation = 0;
        let mut water_destimation = 0;
        let mut light_destimation = 0;
        let mut temperature_destimation = 0;
        let mut humidity_destimation = 0;
        let mut location_destimation = 0;

        let mut seed_int= seed.parse::<i32>().unwrap();
        println!("Working on seed: {:?}", seed_int);
        let mut lines_clone:Vec<String> = lines.clone();

        for line in lines_clone {

            let mut this_line = line.to_string();
    
            let mut char_1 = this_line.chars().nth(0).unwrap();
    
            this_map_name = if char_1.is_alphabetic() {
                this_line.clone()
            } else {
                this_map_name.clone()
            }.to_string();
    
            this_map = if char_1.is_alphabetic() == false {
                this_line.clone()
            } else {
                "none".to_string()
            }.to_string();

            let mut destination_range_start = 0;
            let mut destination_range_end = 0;
            let mut source_range_start = 0;
            let mut source_range_end = 0;
            let mut range_length = 0;

            if this_map.chars().nth(0).unwrap().is_numeric() {
                let mut map_vec:Vec<String> = this_map.split(" ").map(|s| s.to_string()).collect();
                if this_map_name == "seed-to-soil map:" {
                    destination_range_start = map_vec[0].parse::<i32>().unwrap();
                    source_range_start = map_vec[1].parse::<i32>().unwrap();
                    range_length = map_vec[2].parse::<i32>().unwrap();
                    destination_range_end = destination_range_start+range_length;
                    source_range_end = source_range_start+range_length;
                }
                
            }
        }

        output = if location_destimation < output {
            location_destimation
        } else {
            output
        };
        
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