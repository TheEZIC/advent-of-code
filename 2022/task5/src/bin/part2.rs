use task5::{find_min_location, parse_converters, parse_seeds_range, read_input};

fn main() {
    let input = read_input();
    process_part2(input);
}

fn process_part2(lines: Vec<String>) {
    let seeds = parse_seeds_range(&lines[0]);
    let converters = parse_converters(&lines);

    let min_location = find_min_location(&converters, &seeds);

    println!("{}", min_location);
}

