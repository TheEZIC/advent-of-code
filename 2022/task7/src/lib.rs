pub mod card;

use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}