
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
//use std::mem::replace;
use std::collections::HashMap;

fn main() {
    //println!("Begin:");


    let mut output:i64 = 0;
    let computer_killing_multiplier = 1000000;

    let file_path = "input.txt";

    //println!("In file {}", file_path);

    let mut lines: Vec<String> = read_lines(file_path);


    //step 1 - get grid into vector of char vector

    let mut grid: Vec<Vec<char>> = Vec::new();

    for (index, this_line) in lines.iter().enumerate() {
        let this_line_vec: Vec<char> = this_line.chars().collect();
        grid.push(this_line_vec);
    }

    //println!("Starting Grid");
    for (index, y_line) in grid.iter().enumerate() {
        //println!("y axis: {:?}: {:?}", index, y_line);
    }


    //step 2 - check for empty rows and duplicate them

    let mut grid_with_expanded_rows: Vec<Vec<char>> = Vec::new();

    for (index, y_line) in grid.iter().enumerate() {
        let mut row_is_empty = 1;
        grid_with_expanded_rows.push(y_line.to_vec());
        for (index, point) in y_line.iter().enumerate() {
            if *point != '.' {row_is_empty = 0}
        }
        if row_is_empty == 1 {
            for i in 1..computer_killing_multiplier {
                grid_with_expanded_rows.push(y_line.to_vec());
            }
            
        } 
    }

    drop(grid);

    println!("rows expanded sucessfully");
   

    //step 3 - check for empty columns and duplicate them

    //pivot so duplicate rows can be used
    let mut grid_with_expanded_rows__pivot: Vec<Vec<char>> = Vec::new();
    for (index, y_line) in grid_with_expanded_rows.iter().enumerate() {
        if index == 0 {
            for (index, point) in y_line.iter().enumerate() {
                let mut this_line_vec: Vec<char> = Vec::new();
                this_line_vec.push(*point);
                grid_with_expanded_rows__pivot.push(this_line_vec);
            }
        }
        else {
            for (index, point) in y_line.iter().enumerate() {
                grid_with_expanded_rows__pivot[index].push(*point);
            }
        }
    }

    drop(grid_with_expanded_rows);
    println!("pivoted sucessfully");

    //expand rows again
    let size_as_u8: usize = 3145728000;
    let mut grid_with_expanded_columns: Vec<Vec<char>> = Vec::with_capacity(size_as_u8);
    //grid_with_expanded_columns.resize(grid_with_expanded_columns, size_as_u8);
    //grid_with_expanded_columns.try_reserve(size_as_u8); 

    for (index, y_line) in grid_with_expanded_rows__pivot.iter().enumerate() {
        let mut row_is_empty = 1;
        grid_with_expanded_columns.push(y_line.to_vec());
        for (index, point) in y_line.iter().enumerate() {
            if *point != '.' {row_is_empty = 0}
        }
        if row_is_empty == 1 {
            for i in 1..computer_killing_multiplier {
                grid_with_expanded_columns.push(y_line.to_vec());
            }
        }
    }

    drop(grid_with_expanded_rows__pivot);
    println!("columns expanded sucessfully");

    //pivot back
    let mut expended_grid: Vec<Vec<char>> = Vec::new();
    for (index, y_line) in grid_with_expanded_columns.iter().enumerate() {
        if index == 0 {
            for (index, point) in y_line.iter().enumerate() {
                let mut this_line_vec: Vec<char> = Vec::new();
                this_line_vec.push(*point);
                expended_grid.push(this_line_vec);
            }
        }
        else {
            for (index, point) in y_line.iter().enumerate() {
                expended_grid[index].push(*point);
            }
        }
    }

    drop(grid_with_expanded_columns);
    println!("pivoted back sucessfully");

    //println!("expanded grid");
    for (index, y_line) in expended_grid.iter().enumerate() {
        //println!("y axis: {:?}: {:?}", index, y_line);
    }

    //step 3 - find galaxies and distances between them

    let mut galaxies: Vec<Vec<i64>> = Vec::new();
    for (index, y_line) in expended_grid.iter().enumerate() {
        let mut y_axis = index;
        for (index, point) in y_line.iter().enumerate() {
            let mut x_axis = index;
            if *point != '.' {
                //println!("found a galaxy at: {:?} {:?}", y_axis, x_axis);
                let mut this_galaxy: Vec<i64> = Vec::new();
                let y_axis_int:i64 = y_axis.try_into().unwrap();
                let x_axis_int:i64 = x_axis.try_into().unwrap();
                this_galaxy.push(y_axis_int);
                this_galaxy.push(x_axis_int);
                galaxies.push(this_galaxy);
            }
        }
    }

    drop(expended_grid);
    println!("galaxies vector created");

    //println!("galaxies: {:?}: ", galaxies);



    for (index, galaxy) in galaxies.iter().enumerate() {
        //println!("finding pares for: {:?}", galaxy);
        let this_y_axis = galaxy[0];
        let this_x_axis = galaxy[1];
        
        for pair in galaxies.iter().skip(index+1) {
            //println!("check against: {:?}", pair); 
            let check_y_axis = pair[0];
            let check_x_axis = pair[1];

            if check_y_axis < this_y_axis { output = output + (this_y_axis-check_y_axis) }
            else { output = output + (check_y_axis - this_y_axis) }

            if check_x_axis < this_x_axis { output = output + (this_x_axis-check_x_axis) }
            else { output = output + (check_x_axis - this_x_axis) }

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