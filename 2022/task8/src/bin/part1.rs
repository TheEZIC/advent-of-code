use task8::{read_direction_line, read_input, read_path_lines};

fn main() {
    let input = read_input();
    process_part1(&input);
}

fn process_part1(lines: &Vec<String>) {
    let directions = read_direction_line(&lines[0]);
    let paths = read_path_lines(
        &lines
            .iter()
            .skip(2)
            .map(|str| str.to_string())
            .collect::<Vec<String>>()
    );

    let mut current_key = "AAA".to_string();
    let mut count: u32 = 0;

    while current_key != "ZZZ" {
        let (left, right) = paths.get(&current_key).unwrap();
        let direction_index = (count % directions.len() as u32) as usize;
        let direction= directions[direction_index];

        match direction {
            'L' => current_key = left.to_string(),
            'R' => current_key = right.to_string(),
            _ => panic!("Unknown direction")
        }

        count += 1;
    }

    println!("{:?}", count);
}