// #![allow(unused_must_use, dead_code, unused_mut, unused_variables, unused_imports)]

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct TranslatableValue {
    source_range_start: u64,
    source_range_end: u64,
    destination_range_start: u64,
}

#[derive(Debug)]
struct TranslatableCollection {
    translatables: Vec<TranslatableValue>,
}
impl TranslatableCollection {
    fn new(file_name: &str) -> Self {
        let mut tv: Vec<TranslatableValue> = Vec::new();
        let input_file = File::open(file_name).unwrap();
        let reader = BufReader::new(input_file);
        for line in reader.lines() {
            let line_string = line.unwrap();
            let ns: Vec<&str> = line_string.split_whitespace().collect(); 
            let destination_range_start = ns[0].parse::<u64>().unwrap();
            let source_range_start =  ns[1].parse::<u64>().unwrap();
            let range_length = ns[2].parse::<u64>().unwrap();
            let t = TranslatableValue{ 
                source_range_start, 
                source_range_end: source_range_start + range_length - 1, 
                destination_range_start 
            };
            tv.push(t);
        }
        Self { translatables: tv }
    }

    fn translated_value(&self, number: u64) -> u64 {
        for t in &self.translatables {
            if number >= t.source_range_start && number <= t.source_range_end {
                return t.destination_range_start + (number - t.source_range_start);
            }
        }
        number
    }
}

fn main() {
    let mut seeds: Vec<u64> = Vec::new();
    let seed_to_soil = TranslatableCollection::new("seed-to-soil.txt");
    let soil_to_fertilizer = TranslatableCollection::new("soil-to-fertilizer.txt");
    let fertilizer_to_water = TranslatableCollection::new("fertilizer-to-water.txt");
    let water_to_light = TranslatableCollection::new("water-to-light.txt");
    let light_to_temperature = TranslatableCollection::new("light-to-temperature.txt");
    let temperature_to_humidity = TranslatableCollection::new("temperature-to-humidity.txt");
    let humidity_to_location = TranslatableCollection::new("humidity-to-location.txt"); 

    let input_file = File::open("seeds.txt").unwrap();
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        for s in line.unwrap().split_ascii_whitespace() {
            seeds.push(s.parse().unwrap());
        }
    }

    let mut locations: Vec<u64> = Vec::new();
    for s in &seeds {
        let soil = seed_to_soil.translated_value(*s);
        let fertilizer = soil_to_fertilizer.translated_value(soil);
        let water = fertilizer_to_water.translated_value(fertilizer);
        let light = water_to_light.translated_value(water);
        let temperature = light_to_temperature.translated_value(light);
        let humidity = temperature_to_humidity.translated_value(temperature);
        let location = humidity_to_location.translated_value(humidity);

        locations.push(location);
    }

    let mut lowest_location: u64 = 0;
    for i in (0..20).step_by(2) {
        println!("--- {}", i);
        println!("Processing: {} - {}", seeds[i], seeds[i]+seeds[i+1]);
        println!("Processing: {}", seeds[i+1]);
        println!("Processing: {}", (seeds[i]+seeds[i+1]) - seeds[i]);
        for s in seeds[i]..(seeds[i]+seeds[i+1]) {
            let soil = seed_to_soil.translated_value(s);
            let fertilizer = soil_to_fertilizer.translated_value(soil);
            let water = fertilizer_to_water.translated_value(fertilizer);
            let light = water_to_light.translated_value(water);
            let temperature = light_to_temperature.translated_value(light);
            let humidity = temperature_to_humidity.translated_value(temperature);
            let location = humidity_to_location.translated_value(humidity);
            if lowest_location == 0 || location < lowest_location {
                lowest_location = location;
            }
        }
    }

    println!("The lowest location number that corresponds to any of the initial seed numbers is {}", locations.iter().min().unwrap());
    println!("The lowest location number that corresponds to any of the initial seed numbers using ranges is {}", lowest_location);

}
