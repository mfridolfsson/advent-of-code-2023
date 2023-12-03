
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
    let mut lines_vec: Vec<Vec<char>> = Vec::new();

    for line in lines {

        let line_vec: Vec<char> = line.chars().collect();
        lines_vec.push(line_vec);
    
    }

    let mut y_axis = 1;

    for line in lines_vec {

        println!("line {:?} {:?}", y_axis, line);

        let mut x_axis = 1;
        let mut this_number = String::from("");
        let mut is_working_on_number = 0;
        let mut start_x = 0;
        let mut end_x = 0;
        for point in line {
            if point.is_digit(10) == false {
                println!("{:?} is not a number", point); 
            
            }
            if point.is_digit(10) == false && is_working_on_number == 1 {
                is_working_on_number = 0;
                end_x = x_axis-1;
                println!("found {:?} start at {:?} end at {:?}", this_number, start_x, end_x); 
                
                //find if should be counted
                let check_x = 1
                let check_y = 1
                for line in lines_vec {

                //count valids
                if is_valid==1 {
                    let re = Regex::new(r"[^0-9.]").unwrap();
                    let just_numbers = re.replace_all(this_number.as_str(),"");
                    println!("{:?}", just_numbers); 
                    let this_number_int = just_numbers.parse::<i32>().unwrap();
                    output = output + this_number_int;
                }
                this_number = String::from("");
            }


            if point.is_digit(10) {
                println!("{:?} is a digit at {:?} {:?}", point,y_axis, x_axis);
                this_number.push(point);
                if is_working_on_number == 0 {
                    is_working_on_number = 1;
                    start_x = x_axis;
                }
            }
            x_axis = x_axis + 1;
        }
        y_axis = y_axis + 1;
    
    }

    println!("SOLUTION:{:?} ", output);
    
    /* 
    for line in lines {

        y_axis = y_axis + 1;
        let mut x_axis = 0;

        let line_str = line.as_str();
        println!("{}", line_str);
        let line_vec: Vec<char> = line.chars().collect();

        for pos in line_vec {
            x_axis = x_axis + 1;

            if (!pos.is_alphanumeric() && pos != '.') {
                println!("character at: x: {}", x_axis);
                println!("character at: y: {}", y_axis);
            }
        }
    

    }
    */
    
   
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}