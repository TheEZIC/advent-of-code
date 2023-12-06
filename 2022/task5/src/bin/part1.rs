use task5::{find_min_location, parse_converters, parse_seeds, read_input};

fn main() {
    let input = read_input();
    process_part1(input);
}

fn process_part1(lines: Vec<String>) {
    let seeds = parse_seeds(&lines[0]);
    let converters = parse_converters(&lines);

    let min_location = find_min_location(&converters, &seeds);

    println!("{}", min_location);
}
