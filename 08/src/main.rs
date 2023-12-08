// #![allow(unused_must_use, dead_code, unused_mut, unused_variables, unused_imports)]

use regex::Regex;
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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(numbers: Vec<u64>) -> u64 {
    let mut lcm: u64 = 1;
    for number in numbers {
        lcm = lcm * (number) / gcd(lcm, number);
    }
    lcm
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

    let keys: Vec<String> = node_map.keys().cloned().collect();
    let group_of_nodes: Vec<String> = keys.iter()
        .filter(|string| Regex::new(r"..A").unwrap().is_match(string))
        .cloned()
        .collect();
    let mut shortest_paths: Vec<u64> = Vec::new();
    for n in group_of_nodes {
        leaps = 0;
        let mut node_to_consider = &n;
        for character in loop_string.chars().cycle() {
            if Regex::new(r"..Z").unwrap().is_match(&node_to_consider) {
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
        shortest_paths.push(leaps as u64);
    }

    println!("The number of steps it takes before you're only on nodes that end with Z: {}", lcm(shortest_paths));
}
