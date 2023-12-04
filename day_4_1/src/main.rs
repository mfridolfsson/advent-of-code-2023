
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;

fn main() {
    println!("Begin:");

    let file_path = "input.txt";

    println!("In file {}", file_path);

    //let contents = fs::read_to_string(file_path)
    //    .expect("Should have been able to read the file");

    let mut output:i32 = 0;

    let lines = read_lines(file_path);

    for line in lines {
    // line = card

        let mut card_value:i32 = 0;

        let this_line = line.as_str();
        println!("{}", this_line);

        let id_vs_data_vec:Vec<String> = this_line.split(":").map(|s| s.to_string()).collect();
        let id_string = id_vs_data_vec[0].as_str();
        let data_string = id_vs_data_vec[1].as_str();
        println!("id_string: {}", id_string);

        let id_vs_data_vec:Vec<String> = data_string.split("|").map(|s| s.to_string()).collect();
        let winning_numbers_data_raw = id_vs_data_vec[0].as_str();
        let card_numbers_data_raw = id_vs_data_vec[1].as_str();
        println!("winning numbers raw: {}", winning_numbers_data_raw);
        println!("card numbers raw: {}", card_numbers_data_raw);

        //strip out double spaces
        let re = Regex::new(r"(  )").unwrap();
        let winning_numbers_data = re.replace_all(winning_numbers_data_raw ," ");
        let card_numbers_data = re.replace_all(card_numbers_data_raw ," ");
        println!("Winning numbers: {}", winning_numbers_data);
        println!("Card numbers: {}", card_numbers_data);

        let winning_numbers_vec:Vec<String> = winning_numbers_data.split(" ").map(|s| s.to_string()).collect();
        

        for winning_number in winning_numbers_vec {
            let card_numbers_vec:Vec<String> = card_numbers_data.split(" ").map(|s| s.to_string()).collect();
            if winning_number.len() > 0 {
                println!("Checking for winnng Number: {:?}", winning_number);

                for card_number in card_numbers_vec {
                    if card_number.len() > 0 {
                        if card_number == winning_number {
                        println!("Winning Number found!: {:?}", card_number);
                        if card_value == 0 {
                            card_value = 1
                        }
                        else {
                            card_value = card_value * 2;
                        }
                        }
                    }
                }
            }
        }
        
        output = output + card_value;

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