use task6::{calc_solutions_derivative, read_input};

fn main() {
    let lines = read_input();
    process_part1(lines);
}

fn process_part1 (lines: Vec<String>) {
    let times = parse_line(&lines[0], "Time:");
    let distances = parse_line(&lines[1], "Distance:");

    let result = calc_solutions_derivative(&times, &distances);

    println!("answer: {}", result);
}

fn parse_line(line: &String, prefix: &str) -> Vec<u64> {
    return line
        .replace(prefix, "")
        .split_whitespace()
        .filter(|str| !str.is_empty())
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
}

