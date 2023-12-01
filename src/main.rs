// #![allow(unused_must_use, dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut coordinate: i32 = 0;
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let mut i = line.unwrap().chars()
                            .filter(|c| c.is_numeric()).collect::<String>();
        
        i = match i.len() {
            1 => i.repeat(2),
            2 => i,
            _ => format!("{}{}", i.chars().nth(0).unwrap(), i.chars().last().unwrap()),
        };
        
        coordinate += i.parse::<i32>().unwrap();
    }

    println!("The coordinate is: {:?}", coordinate);
}
