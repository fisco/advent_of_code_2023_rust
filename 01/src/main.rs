// #![allow(unused_must_use, dead_code)]

use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let text_to_day: BTreeMap<&str, &str> = BTreeMap::from_iter(
        [("one", "o1e"), ("two", "t2o"), ("three", "th3ee"), ("four", "fo4ur"), ("five", "fi5ve"), ("six", "s6x"),
         ("seven", "se7en"), ("eight", "ei8ht"), ("nine", "ni9ne")]
    );
    let mut calibration: i32 = 0;
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let mut massaged_line = String::from(line.unwrap()); 
        
        for (key, value) in text_to_day.iter() {
            massaged_line = Regex::new(format!(r"(?i){}", key).as_ref()).unwrap()
                            .replace_all(&massaged_line, *value).to_string();
        }; 

        massaged_line = massaged_line.chars()
                            .filter(|c| c.is_numeric()).collect::<String>();
        
        massaged_line = match massaged_line.len() {
            1 => massaged_line.repeat(2),
            2 => massaged_line,
            _ => format!("{}{}", massaged_line.chars().nth(0).unwrap(), massaged_line.chars().last().unwrap()),
        };
        calibration += massaged_line.parse::<i32>().unwrap();
    }

    println!("The calibration value is: {:?}", calibration);
}
