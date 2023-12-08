// #![allow(unused_must_use, dead_code, unused_mut, unused_variables, unused_imports)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}
impl Node {
    fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut loop_string: String = String::new();
    let mut node_map: HashMap<String, Node> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        // Extract data:
        let line_string = line.unwrap();
        if i == 0 {
            loop_string = line_string;
        } else if i > 1 {
            let (node_name, remainder) = line_string.split_once(" = ").unwrap();
            let (left_name, right_name) = (&remainder[1..remainder.len() - 1]).split_once(", ").unwrap();
            let node = Node::new(String::from(left_name), String::from(right_name));
            node_map.insert(node_name.to_string(), node);
        }
    }

    let mut node_to_consider: &str = "AAA";
    let mut leaps: u32 = 0;
    for character in loop_string.chars().cycle() {
        if node_to_consider == "ZZZ" {
            break;
        } else {
            let next_location_possibilities = node_map.get(node_to_consider).unwrap();
            node_to_consider = match character {
                'R' => &next_location_possibilities.right,
                'L' => &next_location_possibilities.left,
                _ => panic!(),
            };
            leaps += 1;
        }
    }
    println!("In part 1, the steps required to reach ZZZ total {}", leaps);
}
