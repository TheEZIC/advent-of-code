use task2::{read_input, CubesBag};

fn main() {
    let lines = read_input();
    process_part1(lines);
}

fn process_part1(lines: Vec<String>) {
    let mut sum: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        if is_game_possible(line) {
            let id = (i as u32) + 1;
            sum += id;
        }
    }

    println!("{}", sum);
}

fn is_game_possible(line: &String) -> bool {
    let (_, rounds_chunk) = line.split_once(": ").unwrap();

    for round in rounds_chunk.split("; ") {
        let mut bag =  CubesBag::new();

        for cube in round.split(", ") {
            let (count_string, color) = cube.split_once(" ").unwrap();
            let count = count_string.parse::<i32>().unwrap();

            match color {
                "red" => bag.red -= count,
                "green" => bag.green -= count,
                "blue" => bag.blue -= count,
                _ => {}
            }
        }

        if bag.is_empty() {
            return false;
        }
    }

    return true;
}
