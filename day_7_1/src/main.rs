
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;
use std::collections::HashMap;

fn main() {
    println!("Begin:");

    let mut bet_hash_map: HashMap<String, i32> = HashMap::new();
    let mut rank_value_hash_map: HashMap<String, i32> = HashMap::new();


    let mut five_of_a_kind_hash: HashMap<String, i32> = HashMap::new();
    let mut four_of_a_kind_hash: HashMap<String, i32> = HashMap::new();
    let mut full_house_hash: HashMap<String, i32> = HashMap::new();
    let mut three_of_a_kind_hash: HashMap<String, i32> = HashMap::new();
    let mut two_pair_hash: HashMap<String, i32> = HashMap::new();
    let mut one_pair_hash: HashMap<String, i32> = HashMap::new();
    let mut high_card_hash: HashMap<String, i32> = HashMap::new();

    let mut output = 0;

    let file_path = "input.txt";

    println!("In file {}", file_path);

    let lines = read_lines(file_path);
    let mut multiplier:i32 = lines.len() as i32;

    println!("raw data: {:?}", lines);

    for this_line in lines {
        let mut hand_vs_bet_vec:Vec<String> = this_line.split(" ").map(|s| s.to_string()).collect();
        let mut hand = hand_vs_bet_vec[0].as_str();
        let mut hand_vec:Vec<char> = Vec::new();

        let bet = hand_vs_bet_vec[1].parse::<i32>().unwrap();
        let mut rank_value:String = String::new();
        bet_hash_map.insert(
            hand.to_string(),
            bet,
        );

        for card in hand.chars() { 
            hand_vec.push(card);
            if card == 'T' {rank_value.push_str("11")}
            else if card == 'J' {rank_value.push_str("12")}
            else if card == 'Q' {rank_value.push_str("13")}
            else if card == 'K' {rank_value.push_str("14")}
            else if card == 'A' {rank_value.push_str("15")}
            else {rank_value.push_str("0"); rank_value.push_str(&card.to_string()) };
        }
        rank_value_hash_map.insert(
            hand.to_string(),
            rank_value.parse::<i32>().unwrap(),
        );
        hand_vec.sort();

        
        if hand_vec[0] == hand_vec[4] {
            five_of_a_kind_hash.insert(
                hand.to_string(),
                rank_value.parse::<i32>().unwrap(),
            );
        }
        else if (hand_vec[0] == hand_vec[3] || hand_vec[1] == hand_vec[4]) {
            four_of_a_kind_hash.insert(
                hand.to_string(),
                rank_value.parse::<i32>().unwrap(),
            );
        }
        else if (hand_vec[0] == hand_vec[2] && hand_vec[3] == hand_vec[4]) || (hand_vec[2] == hand_vec[4] && hand_vec[0] == hand_vec[1]) || (hand_vec[1] == hand_vec[2] && hand_vec[3] == hand_vec[4]) {
            full_house_hash.insert(
                hand.to_string(),
                rank_value.parse::<i32>().unwrap(),
            );
        }
        else if (hand_vec[0] == hand_vec[2]) || (hand_vec[2] == hand_vec[4]) || (hand_vec[1] == hand_vec[3]) {
            three_of_a_kind_hash.insert(
                hand.to_string(),
                rank_value.parse::<i32>().unwrap(),
            );
        }
        else if (hand_vec[0] == hand_vec[1]) && (hand_vec[2] == hand_vec[3] || hand_vec[3] == hand_vec[4]) || (hand_vec[4] == hand_vec[3]) && (hand_vec[1] == hand_vec[2] || hand_vec[3] == hand_vec[2])
        || (hand_vec[1] == hand_vec[2]) && (hand_vec[3] == hand_vec[4]) {
            two_pair_hash.insert(
                hand.to_string(),
                rank_value.parse::<i32>().unwrap(),
            );
        }
        else if hand_vec[0] != hand_vec[1] && hand_vec[1] != hand_vec[2] && hand_vec[2] != hand_vec[3] && hand_vec[3] != hand_vec[4] {
            high_card_hash.insert(
                hand.to_string(),
                rank_value.parse::<i32>().unwrap(),
            );
        }
        else {one_pair_hash.insert(
            hand.to_string(),
            rank_value.parse::<i32>().unwrap(),
        );}

    }

    let mut five_of_a_kind_sorted: Vec<_> = five_of_a_kind_hash.iter().collect();
    five_of_a_kind_sorted.sort_by_key(|a| a.1);
    five_of_a_kind_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("five_of_a_kind_vec_sorted: {:?}", five_of_a_kind_sorted);
    for hand in five_of_a_kind_sorted {
        
        let mut this_bet:i32 = 0;
        match bet_hash_map.get(hand.0) {
            Some(&ref bet) => this_bet = *bet,
            _  => println!("No bet data!"),
        }
        //println!("hand:{:?} ", hand.0);
        //println!("bet:{:?} ", this_bet);
        //println!("multiplier:{:?} ", multiplier);
        output = output + (this_bet * multiplier);
        multiplier = multiplier - 1;
        //println!("output:{:?} ", output);
    }

    let mut four_of_a_kind_sorted: Vec<_> = four_of_a_kind_hash.iter().collect();
    four_of_a_kind_sorted.sort_by_key(|a| a.1);
    four_of_a_kind_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("four_of_a_kind_sorted: {:?}", four_of_a_kind_sorted);
    for hand in four_of_a_kind_sorted {
        
        let mut this_bet:i32 = 0;
        match bet_hash_map.get(hand.0) {
            Some(&ref bet) => this_bet = *bet,
            _  => println!("No bet data!"),
        }
        //println!("hand:{:?} ", hand.0);
        //println!("bet:{:?} ", this_bet);
        //println!("multiplier:{:?} ", multiplier);
        output = output + (this_bet * multiplier);
        multiplier = multiplier - 1;
       //println!("output:{:?} ", output);
    }

    let mut full_house_vec_sorted: Vec<_> = full_house_hash.iter().collect();
    full_house_vec_sorted.sort_by_key(|a| a.1);
    full_house_vec_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("full_house_vec_sorted_sorted: {:?}", full_house_vec_sorted);
    for hand in full_house_vec_sorted {
        
        let mut this_bet:i32 = 0;
        match bet_hash_map.get(hand.0) {
            Some(&ref bet) => this_bet = *bet,
            _  => println!("No bet data!"),
        }
        //println!("hand:{:?} ", hand.0);
        //println!("bet:{:?} ", this_bet);
        //println!("multiplier:{:?} ", multiplier);
        output = output + (this_bet * multiplier);
        multiplier = multiplier - 1;
        //println!("output:{:?} ", output);
  }

   let mut three_of_a_kind_vec_sorted: Vec<_> = three_of_a_kind_hash.iter().collect();
   three_of_a_kind_vec_sorted.sort_by_key(|a| a.1);
   three_of_a_kind_vec_sorted.sort_by(|a, b| b.1.cmp(a.1));
   println!("three_of_a_kind_vec_sorted: {:?}", three_of_a_kind_vec_sorted);
   for hand in three_of_a_kind_vec_sorted {
       
       let mut this_bet:i32 = 0;
       match bet_hash_map.get(hand.0) {
           Some(&ref bet) => this_bet = *bet,
           _  => println!("No bet data!"),
       }
       //println!("hand:{:?} ", hand.0);
       //println!("bet:{:?} ", this_bet);
       //println!("multiplier:{:?} ", multiplier);
       output = output + (this_bet * multiplier);
       multiplier = multiplier - 1;
       //println!("output:{:?} ", output);
   }

    let mut two_pair_hash_sorted: Vec<_> = two_pair_hash.iter().collect();
    two_pair_hash_sorted.sort_by_key(|a| a.1);
    two_pair_hash_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("two_pair_vec_sorted_sorted: {:?}", two_pair_hash_sorted);
    for hand in two_pair_hash_sorted {
        
        let mut this_bet:i32 = 0;
        match bet_hash_map.get(hand.0) {
            Some(&ref bet) => this_bet = *bet,
            _  => println!("No bet data!"),
        }
        //println!("hand:{:?} ", hand.0);
        //println!("bet:{:?} ", this_bet);
        //println!("multiplier:{:?} ", multiplier);
        output = output + (this_bet * multiplier);
        multiplier = multiplier - 1;
        //println!("output:{:?} ", output);
    }

    let mut one_pair_sorted: Vec<_> = one_pair_hash.iter().collect();
    one_pair_sorted.sort_by_key(|a| a.1);
    one_pair_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("one_pair_vec_sorted_sorted: {:?}", one_pair_sorted);
    for hand in one_pair_sorted {
        
        let mut this_bet:i32 = 0;
        match bet_hash_map.get(hand.0) {
            Some(&ref bet) => this_bet = *bet,
            _  => println!("No bet data!"),
        }
        //println!("hand:{:?} ", hand.0);
        //println!("bet:{:?} ", this_bet);
        //println!("multiplier:{:?} ", multiplier);
        output = output + (this_bet * multiplier);
        multiplier = multiplier - 1;
        //println!("output:{:?} ", output);
    }
    
    let mut high_card_sorted: Vec<_> = high_card_hash.iter().collect();
    high_card_sorted.sort_by_key(|a| a.1);
    high_card_sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("high_card_sorted_sorted: {:?}", high_card_sorted);
    for hand in high_card_sorted {
        
        let mut this_bet:i32 = 0;
        match bet_hash_map.get(hand.0) {
            Some(&ref bet) => this_bet = *bet,
            _  => println!("No bet data!"),
        }
        println!("hand:{:?} ", hand.0);
        println!("bet:{:?} ", this_bet);
        println!("multiplier:{:?} ", multiplier);
        output = (this_bet * multiplier)+output;
        println!("output:{:?} ", output);
        multiplier = multiplier - 1;
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