
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

    let mut output:i32 = 1;

    let lines = read_lines(file_path);

    
    let mut mutiply_value:i32 = 0;

    //let mut cards_hash_map = HashMap::new();
    let mut run_list: Vec<i32> = Vec::new();
    let mut re_run_list: Vec<i32> = Vec::new();
    let mut this_re_run_list: Vec<i32> = Vec::new();
    let mut card_value:i32 = 0;
    
    let mut times = &lines[0];
    let mut distances = &lines[1];
    println!("times string: {}", times);
    println!("distances string: {}", distances);

    let times_id_vs_data_vec:Vec<String> = times.split(":").map(|s| s.to_string()).collect();
    let distances_id_vs_data_vec:Vec<String> = distances.split(":").map(|s| s.to_string()).collect();
    let mut times_just_data = &times_id_vs_data_vec[1];
    let mut distance_just_data = &distances_id_vs_data_vec[1];
    println!("just times: {}", times_just_data);
    println!("just distance: {}", distance_just_data);

    let mut times_clean_vec:Vec<String> = times_just_data.split(" ").map(|s| s.to_string()).collect();
    times_clean_vec.retain(|x| x != "");
    println!("clean times: {:?}", times_clean_vec);

    let mut distances_clean_vec:Vec<String> = distance_just_data.split(" ").map(|s| s.to_string()).collect();
    distances_clean_vec.retain(|x| x != "");
    println!("clean distances: {:?}", distances_clean_vec);


    let mut loop_count = distances_clean_vec.len()-1;
    println!("races to test: {:?}", loop_count+1);

    for n in 0..=loop_count {
        let this_time:i32 = times_clean_vec[n].parse::<i32>().unwrap();
        let this_distance:i32 = distances_clean_vec[n].parse::<i32>().unwrap();
        let mut win_count = 0;
        println!("time: {:?}", this_time);
        println!("distance: {:?}", this_distance);
        for h in 0..=this_time {
            let mut distance_traveled = 0;
            println!("held for {:?} seconds", h);
            distance_traveled = h * (this_time-h);
            println!("boat traveled for {:?} millimeters",distance_traveled);
            if distance_traveled > this_distance {
                println!("winnah");
                win_count = win_count +1;
            }
        }
        println!("winna races: {:?}", win_count);
        output = output * win_count;
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