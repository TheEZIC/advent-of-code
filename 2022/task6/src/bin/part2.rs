use task6::{calc_solutions_count, read_input};

fn main() {
    let lines = read_input();
    process_part2(lines);
}

fn process_part2 (lines: Vec<String>) {
    let time = parse_line(&lines[0], "Time:");
    let distance = parse_line(&lines[1], "Distance:");

    let result = calc_solutions_count(&time, &distance);

    println!("answer: {}", result);
}

fn parse_line(line: &String, prefix: &str) -> u64 {
    let r =  line
        .replace(prefix, "")
        .split_whitespace()
        .filter(|str| !str.is_empty())
        .collect::<String>();

    return r.parse::<u64>().unwrap()
}