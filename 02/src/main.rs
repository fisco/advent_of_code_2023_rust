#![allow(unused_must_use, dead_code)]

// use regex::Regex;
// use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn game_would_have_been_possible(final_score: &HashMap<&str, u32>) -> bool {
    if *final_score.get("red").unwrap() > 12 { return false; };
    if *final_score.get("blue").unwrap() > 14 { return false; };
    if *final_score.get("green").unwrap() > 13 { return false; };
    true
}

fn main() {
    let mut total: i32 = 0;
    let mut game_power_total: u32 = 0;
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    

    for line in reader.lines() {
        let mut cube_maxes: HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let line_string = line.unwrap(); 
        let (game_part, result_part) = line_string.split_once(':').unwrap();         
        let showings = result_part.split(r"; ");

        for showing in showings {
            let cubes_in_showing = showing.split(r", ");
            for cubes in cubes_in_showing {
                let (number, color) = cubes.trim().split_once(' ').unwrap();
                if number.parse::<u32>().unwrap() > *cube_maxes.get(color.trim()).unwrap() {
                  cube_maxes.insert(color.trim(), number.parse().unwrap());
                }
            }
        }
    
        if game_would_have_been_possible(&cube_maxes) {
            let (_, game_number_string) = game_part.split_once(' ').unwrap();
            total += game_number_string.trim().parse::<i32>().unwrap();
        }

        let mut game_power: u32 = 1;
        for (key, _value) in cube_maxes.iter() {
            game_power *= cube_maxes.get(key).unwrap();
        }
        game_power_total += game_power;
    }
    
    println!("The possible game total value is: {}", total);
    println!("The possible game power total is: {}", game_power_total);
}
