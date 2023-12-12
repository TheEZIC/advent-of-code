use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn solve(lines: &Vec<String>, backwards: bool) {
    let sum: i32 = lines.iter()
        .map(|line| parse_numbers_line(line))
        .map(|numbers| extrapolate_numbers(&numbers, backwards))
        .sum();

    println!("{}", sum);
}

fn parse_numbers_line(line: &String) -> Vec<i32> {
    return line
        .split_whitespace()
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn extrapolate_numbers(numbers: &Vec<i32>, backwards: bool) -> i32 {
    let next_numbers = (1..numbers.len())
        .map(|i| numbers[i] - numbers[i - 1])
        .collect::<Vec<i32>>();

    let all_zeros = next_numbers.iter().all(|n| *n == 0);

    return if all_zeros  {
        *numbers.first().unwrap()
    } else if backwards {
        *numbers.first().unwrap() - extrapolate_numbers(&next_numbers, true)
    } else {
        *numbers.last().unwrap() + extrapolate_numbers(&next_numbers, false)
    }
}