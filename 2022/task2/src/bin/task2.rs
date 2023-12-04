use task2::{read_input, CubesBag};

fn main() {
    let lines = read_input();
    process_part2(lines);
}

fn process_part2(lines: Vec<String>) {
    let mut sum: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        sum += calc_sum(line);
    }

    println!("{}", sum);
}

fn calc_sum(line: &String) -> u32 {
    let mut bag =  CubesBag::empty();
    let (_, rounds_chunk) = line.split_once(": ").unwrap();

    for round in rounds_chunk.split("; ") {
        for cube in round.split(", ") {
            let (count_string, color) = cube.split_once(" ").unwrap();
            let count = count_string.parse::<i32>().unwrap();

            match color.trim() {
                "red" => bag.red = bag.red.max(count),
                "green" => bag.green = bag.green.max(count),
                "blue" => bag.blue = bag.blue.max(count),
                _ => {}
            }
        }
    }

    return (bag.red * bag.green * bag.blue) as u32;
}
