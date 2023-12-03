#![allow(unused_must_use, dead_code, unused_mut, unused_variables)]

use regex::Regex;
use range_ext::intersect::Intersect;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total: i32 = 0;
    let mut gear_ratios_sum = 0;
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut lines_vector: Vec<String> = Vec::new();
    let re = Regex::new("[0-9]+").unwrap();
    let zapper = Regex::new(r"[\d\.]+").unwrap();
    let gear_re = Regex::new(r"\*").unwrap();

    for line in reader.lines() {
        lines_vector.push(line.unwrap());
    }
    let length_of_strings = lines_vector.len();

    for i in 0..lines_vector.len() {
        let previous_line_number = max(i as i32 - 1, 0);
        let next_line_number = min(i+1, lines_vector.len()-1);
        let matches_iter = re.find_iter(&lines_vector[i]);
        for single_match in matches_iter {
            let first_pos_to_consider = max(single_match.start() as i32 - 1, 0);
            let last_pos_to_consider = min(single_match.end()+1, length_of_strings-1);
            let consideration_string = format!( "{}{}{}", 
            &lines_vector[previous_line_number as usize][first_pos_to_consider as usize..last_pos_to_consider], 
            &lines_vector[i][first_pos_to_consider as usize..last_pos_to_consider], 
            &lines_vector[next_line_number][first_pos_to_consider as usize..last_pos_to_consider] );
            let consideration_string = zapper.replace_all(&consideration_string, "");
            match consideration_string.len() {
                0 => {},
                _ => { total += single_match.as_str().parse::<i32>().unwrap(); },
            }
        } 
    }

    for i in 0..lines_vector.len() {
        let matches_iter = gear_re.find_iter(&lines_vector[i]);
        for single_match in matches_iter {
            // See if there are > 2 digits in the target zone.
            let target_zone_columns_start = max(single_match.start() as i32 - 1, 0) as usize;
            let target_zone_columns_end = min(single_match.end()+1, length_of_strings-1);
            let target_zone_range = target_zone_columns_start..target_zone_columns_end;
            let mut numbers_in_target_zone: Vec<u32> = Vec::new();
            for j in max(i as u32 - 1, 0) as usize..=min(i + 1, lines_vector.len() - 1) {
                // Get all numbers and record those that intersect with the target zone.
                for digits in re.find_iter(&lines_vector[j]) {
                    let current_range = digits.start()..digits.end();
                    if target_zone_range.does_intersect(&current_range) {
                        numbers_in_target_zone.push(digits.as_str().parse::<i32>().unwrap() as u32); 
                    }
                }
            }
            if numbers_in_target_zone.len() > 1 {
                gear_ratios_sum += numbers_in_target_zone.iter().product::<u32>();
            }

        }
    }
    
    println!("The sum of the part numbers is: {}", total);
    println!("The sum of all gear ratios is: {}", gear_ratios_sum);
}
