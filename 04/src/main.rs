// #![allow(unused_must_use, dead_code, unused_mut, unused_variables, unused_imports)]

use core::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut card_values: Vec<u32> = Vec::new();
    let mut card_number_of_matching_numbers: Vec<u32> = Vec::new();
    let mut cards_you_have: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line_string = line.unwrap(); 
        let (game_part, winners_part) = line_string.split_once(" | ").unwrap();
        let mut card_value = 0;
        let mut nm = 0;
        for w in winners_part.trim().split_ascii_whitespace() {
            let (_, card_numbers_part) = game_part.split_once(": ").unwrap();
            for n in card_numbers_part.trim().split_whitespace() {
                if n.parse::<u32>() == w.parse::<u32>() {
                    nm += 1;
                    match card_value {
                        0 => { card_value = 1 },
                        _ => { card_value *= 2 },
                    }
                }
            }
        }
        card_values.push(card_value);
        card_number_of_matching_numbers.push(nm);
    }
    
    for _i in 0..card_number_of_matching_numbers.len() {
        cards_you_have.push(1);
    }
    for i in 0..card_number_of_matching_numbers.len() {
        if card_number_of_matching_numbers[i] > 0 {
            for j in min(i+1, card_number_of_matching_numbers.len()-1)..min(i+1+card_number_of_matching_numbers[i] as usize, card_number_of_matching_numbers.len()) {
                cards_you_have[j] += cards_you_have[i];
            }
        }
    }

    println!("The simple value of all the cards is: {}", card_values.iter().sum::<u32>());
    println!("The number of cards you have is: {}", cards_you_have.iter().sum::<u32>());
}
