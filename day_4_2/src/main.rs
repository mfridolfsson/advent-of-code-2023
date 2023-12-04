
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

    let lines = read_lines(file_path);

    
    let mut mutiply_value:i32 = 0;

    let mut cards_hash_map = HashMap::new();
    let mut run_list: Vec<i32> = Vec::new();
    let mut re_run_list: Vec<i32> = Vec::new();
    let mut this_re_run_list: Vec<i32> = Vec::new();
    let mut card_value:i32 = 0;
    


    for line in lines {
    // line = card

        let this_line = line.as_str();
        //println!("{}", this_line);

        let id_vs_data_vec:Vec<String> = this_line.split(":").map(|s| s.to_string()).collect();
        let id_string = id_vs_data_vec[0].as_str();
        let data_string = id_vs_data_vec[1].as_str();
        //println!("id_string: {}", id_string);

        
        let re = Regex::new(r"[^0-9.]").unwrap();
        let just_id_value = re.replace_all(id_string ,"");
        let card_id_int = just_id_value.parse::<i32>().unwrap();
        //println!("card id: {:?}", card_id_int);

        cards_hash_map.insert(
            card_id_int,
            data_string.to_string(),
        );

        run_list.push(card_id_int);

    }

    let mut should_loop = 1;

    let mut is_first_loop = 1;

    while should_loop == 1 {

        //println!("Run List: {:?}", run_list);
        for card_id in run_list {
            let mut card_value:i32 = 0;
            let mut this_card_data = "";
            match cards_hash_map.get(&card_id) {
                Some(&ref card_data) => this_card_data = card_data,
                _  => println!("No data!"),
            }
        
            //println!("card data: {:?}", this_card_data);
    
            this_re_run_list = fn_run_card(this_card_data, &card_id);
            output = output + 1;
            card_value = this_re_run_list.len() as i32;
            re_run_list.append(&mut this_re_run_list);
            //println!("rerun list len: {:?}", re_run_list.len());
    
            
            //println!("Card Value: {:?}", card_value);
        }
        run_list = Vec::new();
        run_list.append(&mut re_run_list);

        println!("-----------------------------RERUNNING-----------------------------------------");
        println!("New Run List len: {:?}", run_list.len());


        let minValue = run_list.iter().min();
        match minValue {
            Some(min) => println!( "Min value: {}", min ),
            None      => println!( "Vector is empty" ),
        }
        

        if run_list.len() > 0 {
            should_loop = 1;
        }
        else {
            should_loop = 0;
        }
    }


    println!("SOLUTION:{:?} ", output);
    
        /*
    for card_id in run_list {
        let mut card_value:i32 = 0;
        let mut this_card_data = "";
        match cards_hash_map.get(&card_id) {
            Some(&ref card_data) => this_card_data = card_data,
            _  => println!("No data!"),
        }
    
        println!("card data: {:?}", this_card_data);

        this_re_run_list = fn_run_card(this_card_data, &card_id);
        card_value = this_re_run_list.len() as i32;
        re_run_list.append(&mut this_re_run_list);
        println!("rerun list: {:?}", re_run_list);

        output = output + card_value;
        println!("Card Value: {:?}", card_value);
    }

    run_list = re_run_list;
     */
    
   
}


fn fn_run_card(this_card_data:&str, card_id:&i32) -> Vec<i32> {
    
    let mut card_value:i32 = 0;
    let mut re_run_list: Vec<i32> = Vec::new();

    //println!("running card: {:?}", card_id);

    let id_vs_data_vec:Vec<String> = this_card_data.split("|").map(|s| s.to_string()).collect();
    let winning_numbers_data_raw = id_vs_data_vec[0].as_str();
    let card_numbers_data_raw = id_vs_data_vec[1].as_str();

    //println!("winning numbers raw: {}", winning_numbers_data_raw);
    //println!("card numbers raw: {}", card_numbers_data_raw);

    //strip out double spaces
    let re = Regex::new(r"(  )").unwrap();
    let winning_numbers_data = re.replace_all(winning_numbers_data_raw ," ");
    let card_numbers_data = re.replace_all(card_numbers_data_raw ," ");
    //println!("Winning numbers: {}", winning_numbers_data);
    //println!("Card numbers: {}", card_numbers_data);

    let winning_numbers_vec:Vec<String> = winning_numbers_data.split(" ").map(|s| s.to_string()).collect();
    

    for winning_number in winning_numbers_vec {
        let card_numbers_vec:Vec<String> = card_numbers_data.split(" ").map(|s| s.to_string()).collect();
        if winning_number.len() > 0 {
            //println!("Checking for winnng Number: {:?}", winning_number);

            for card_number in card_numbers_vec {
                if card_number.len() > 0 {
                    if card_number == winning_number {
                    //println!("Winning Number found!: {:?}", card_number);
                    card_value = card_value + 1;
                    }
                }
            }
        }
    }

    let mut next_card_id_int = card_id + 1;
    let card_to_copy_to = next_card_id_int + card_value;
    let mut copy_card = next_card_id_int;

    while copy_card < card_to_copy_to
    {
        //println!("copy card: {:?}", copy_card);
        re_run_list.push(copy_card);
        copy_card = copy_card + 1;
      
    }

    //println!("rerun list: {:?}", re_run_list);
    re_run_list
}

// borrowed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}