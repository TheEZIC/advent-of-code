use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn read_card_set(numbers_set_string: String) -> Vec<u32> {
    let numbers_strings: Vec<String> = numbers_set_string
        .split(" ")
        .filter(|str| !str.is_empty())
        .map(|str| str.trim().to_string())
        .collect();

    let mut numbers: Vec<u32> = Vec::new();

    for number_string in numbers_strings.iter() {
        let number = number_string.parse::<u32>().unwrap();
        numbers.push(number);
    }

    return numbers;
}