#![allow(unused_must_use, dead_code, unused_mut, unused_variables, unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    cards_with_wildcard: [u8; 5],
    cards_for_comparison_with_wildcard: [u8; 5],
    bid: u16,
    strength: u8,
    strength_with_wildcard: u8,
    wildcard: Option::<u8>,
}
impl Hand {
    fn new(s: &str, bid: u16, wildcard: Option::<u8>) -> Self {
        let mut cards: [u8; 5] = [0; 5];
        for (i, ch) in s.chars().enumerate() {
            cards[i] = match ch {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => ch.to_digit(10).unwrap() as u8,
            };
        };

        let strength = Hand::determine_strength(cards);
        let mut cards_with_wildcard = cards;
        let mut strength_with_wildcard = strength;
        match wildcard {
            Some(c) => {
                let mut cards_vec: Vec<u8> = Vec::from(cards);
                let filtered_cards: Vec<u8> = cards_vec.iter().filter(|x| **x != c).cloned().collect();
                // Get card with most duplicates to find what to replace:
                let mut counts: HashMap<u8, usize> = HashMap::new();
                    for x in &filtered_cards {
                    *counts.entry(*x).or_insert(0) += 1;
                }
                let most_frequent_value = counts.iter().max_by_key(|&(_, count)| count);
                let mut max_card = match most_frequent_value {
                    Some(c) => *c.0,
                    None => 11,
                };

                cards_vec.iter_mut().for_each(|x| if *x == c { *x = max_card; });
                cards_with_wildcard = cards_vec.try_into().unwrap();
                strength_with_wildcard = Hand::determine_strength(cards_with_wildcard);
            },
            None => {
                cards_with_wildcard = cards;
                strength_with_wildcard = strength;
            },
        }

        let mut cards_for_comparison_with_wildcard = cards;
        match wildcard {
            Some(c) => {
                let mut cards_vec: Vec<u8> = Vec::from(cards);
                cards_vec.iter_mut().for_each(|x| if *x == 11 { *x = 1; });
                cards_for_comparison_with_wildcard = cards_vec.try_into().unwrap();
            },
            None => {
            },
        }

        Self { cards, cards_with_wildcard, cards_for_comparison_with_wildcard, bid, strength, strength_with_wildcard, wildcard }
    }

    // Returns without_wildcard, with_wildcard
    fn beats(&self, other_hand: &Hand) -> (bool, bool) {
        let mut result_without_wildcard: bool = self.strength > other_hand.strength;
        let mut result_with_wildcard: bool = self.strength_with_wildcard > other_hand.strength_with_wildcard;
        if self.strength == other_hand.strength {
            for i in 0..5 {
                if self.cards[i] != other_hand.cards[i] {
                    if self.cards[i] > other_hand.cards[i] { 
                        result_without_wildcard = true;
                    } else {
                        result_without_wildcard = false;
                    }
                    break;
                }
            }
        }
        if self.strength_with_wildcard == other_hand.strength_with_wildcard {            
            for i in 0..5 {
                if self.cards_for_comparison_with_wildcard[i] != other_hand.cards_for_comparison_with_wildcard[i] {
                    if self.cards_for_comparison_with_wildcard[i] > other_hand.cards_for_comparison_with_wildcard[i] { 
                        result_with_wildcard = true;
                    } else {
                        result_with_wildcard = false;
                    }
                    break;
                }
            }
        }
        (result_without_wildcard, result_with_wildcard)
    }

    fn determine_strength(cards: [u8; 5]) -> u8 {
        let mut dups: HashMap<u8, u8> = HashMap::new();

        for i in cards {
            dups.entry(i).or_insert(0); 
            if let Entry::Occupied(mut entry) = dups.entry(i) {
                entry.insert(*entry.get() + 1); 
            }
        }

        let mut values: Vec<u8> = dups.into_iter().map(|(_, v)| v).collect();
        values.sort();
        values.reverse();
        let values_str: String = values.into_iter().map(|i| i.to_string()).collect::<String>();

        match values_str.as_str() {
            "11111" => 1,
            "2111" => 2,
            "221" => 3,
            "311" => 4,
            "32" => 5,
            "41" => 6,
            "5" => 7,
            _ => panic!(),
        }
    }
}

fn find_index_with_wildcard(vec: &Vec<Hand>, h: &Hand) -> Option<usize>
{
    for (i, item) in vec.iter().enumerate() {
        if item.beats(h).1 {
            return Some(i);
        }
    }
    None
}

fn find_index_without_wildcard(vec: &Vec<Hand>, h: &Hand) -> Option<usize>
{
    for (i, item) in vec.iter().enumerate() {
        if item.beats(h).0 {
            return Some(i);
        }
    }
    None
}

fn main() {
    let mut sorted_hands: Vec<Hand> = Vec::new();
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        // Extract data:
        let line_string = String::from(line.unwrap());
        let mut data_iter = line_string.split_ascii_whitespace();
        let words = data_iter.collect::<Vec<_>>();
        // Create hand:
        let hand = Hand::new(words[0], words[1].parse::<u16>().unwrap(), Some(11));

        let insert_index = find_index_without_wildcard(&sorted_hands, &hand);
        match insert_index {
            Some(i) => { sorted_hands.insert(i, hand) },
            None => { sorted_hands.push(hand) },
        }
    }

    let mut winnings = 0;
    for (i, element) in sorted_hands.iter().enumerate() {
        winnings += (i as u32 +1) * element.bid as u32;
    }
    let mut winnings_with_wildcard = 0;

    println!("The total of winnings for part 1 is: {}", winnings);
    println!("The total of winnings for part 1 should be: 247823654");

    let mut sorted_hands_with_wildcard: Vec<Hand> = Vec::new();
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        // Extract data:
        let line_string = String::from(line.unwrap());
        let mut data_iter = line_string.split_ascii_whitespace();
        let words = data_iter.collect::<Vec<_>>();
        // Create hand:
        let hand = Hand::new(words[0], words[1].parse::<u16>().unwrap(), Some(11));

        let insert_index = find_index_with_wildcard(&sorted_hands_with_wildcard, &hand);
        match insert_index {
            Some(i) => { sorted_hands_with_wildcard.insert(i, hand) },
            None => { sorted_hands_with_wildcard.push(hand) },
        }
    }

    let mut winnings_with_wildcard = 0;
    for (i, element) in sorted_hands_with_wildcard.iter().enumerate() {
        winnings_with_wildcard += (i as u32 +1) * element.bid as u32;
    }

    println!("The total of winnings for part 2 is: {}", winnings_with_wildcard);

}
