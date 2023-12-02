
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

    let lines = read_lines(file_path);
    
    let mut output = 0;

    for line in lines {

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        let this_line = line.as_str();
        println!("{}", this_line);

        let id_vs_data_vec:Vec<String> = this_line.split(":").map(|s| s.to_string()).collect();
        let id_string = id_vs_data_vec[0].as_str();;
        let data_string = id_vs_data_vec[1].as_str();;

        println!("id_string: {}", id_string);
        println!("data_string: {}", data_string);
        let re = Regex::new(r"[^0-9.]").unwrap();
        let just_id_value = re.replace_all(id_string ,"");
        let game_id_int = just_id_value.parse::<i32>().unwrap();
        println!("game_id: {:?}", game_id_int);

        let remove_space = data_string.replace(" ", "");
        let just_data = remove_space.replace(";", ",");
        println!("Just data: {}", just_data);
        let just_data_vec:Vec<String> = just_data.split(",").map(|s| s.to_string()).collect();
        println!("Just data as vec: {:?}", just_data_vec);

        for draw in just_data_vec {
            println!("value: {:?}", draw);
            let draw_str = draw.as_str();
            let draw_vec: Vec<char> = draw.chars().collect();
            
            let re = Regex::new(r"[^0-9.]").unwrap();
            let just_value = re.replace_all(draw_str ,"");
            let value_int = just_value.parse::<i32>().unwrap();
            println!("value: {:?}", value_int);

            let re = Regex::new(r"[0-9.]").unwrap();
            let just_colour = re.replace_all(draw_str ,"");
            println!("colour: {:?}", just_colour);  

            if (just_colour == "red" && value_int > min_red) {
                min_red = value_int;
                println!("min red: {}", min_red);
            }
            else if (just_colour == "green" && value_int > min_green) {
                min_green = value_int;
                println!("min green: {}", min_green);
            }
            else if (just_colour == "blue" && value_int > min_blue) {
                min_blue = value_int;
                println!("min blue: {}", min_blue);
            }
            
        }

        let power = min_red * min_blue * min_green;
        output = output + power;
        println!("running output: {}", output);
    }
    
    println!("output: {}", output);
   
}

// borroewed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}