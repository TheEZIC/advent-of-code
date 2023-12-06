use std::collections::{HashSet};
use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn parse_seeds_range(line: &String) -> Vec<u64> {
    let seeds = parse_seeds(line);
    let mut set = HashSet::<u64>::new();

    for chunk in seeds.chunks(2) {
        let min = chunk[0];
        let max = min + chunk[1];

        for seed in min..max {
            set.insert(seed);
        }
    }

    return set.into_iter().collect();
}

pub fn parse_seeds(line: &String) -> Vec<u64> {
    let (_, seeds_chunk) = line.split_once(":").unwrap();

    let seeds: Vec<u64> = seeds_chunk
        .trim()
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect();

    return seeds;
}

pub fn find_min_location(converters: &Converters, seeds: &Vec<u64>) -> u64 {
    let mut min_location = u64::MAX;

    for seed in seeds {
        let mut location = *seed;

        for ranges in converters.iter() {
            for range in ranges.iter() {
                let (destination, source, len) = *range;

                let min = source;
                let max = min + len;

                if location >= min && location <= max {
                    location = destination + (location - source);
                    break;
                }
            }
        }

        min_location = min_location.min(location);
    }

    return min_location;
}

pub fn parse_converters(lines: &Vec<String>) -> Converters {
    let mut converters: Converters = Vec::new();

    for line in lines.iter() {
        if line.ends_with("map:") {
            converters.push(Vec::new());
        } else if line.len() != 0 {
            if converters.len() == 0 {
                continue;
            }

            let data: Vec<u64> = line
                .split_whitespace()
                .map(|str| str.parse::<u64>().unwrap())
                .collect();

            let list =  converters.last_mut().unwrap();
            let converter_item = (data[0], data[1], data[2]);

            list.push(converter_item);
        }
    }

    return converters;
}

pub type ConverterItem = (u64, u64, u64);

pub type Converters = Vec<Vec<ConverterItem>>;