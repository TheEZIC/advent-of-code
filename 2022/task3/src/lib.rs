use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn input_to_matrix(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();

        matrix.push(chars);
    }

    return matrix;
}

pub fn check_digit_char(char: &char) -> bool {
    let digit_parts: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

    return digit_parts.contains(&char);
}

pub fn set_skip_current(skip_current: &mut bool, char: &char) {
    if *skip_current == false {
        return;
    }

    *skip_current = *char == '.' || check_digit_char(&char);
}