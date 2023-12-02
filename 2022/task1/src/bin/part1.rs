use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .collect::<Vec<&str>>();

    let mut sum: i32 = 0;

    for line in lines {
        sum += compute_calibration_value(String::from(line));
    }

    println!("{}", sum);
}

fn compute_calibration_value(str: String) -> i32 {
    let digit_parts: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut first_digit: Option<String> = Option::None;
    let mut last_digit: Option<String> = Option::None;

    let chars: Vec<char> = str.chars().collect();

    for char in chars {
        let is_digit_part = digit_parts.contains(&char);

        if is_digit_part {
            if first_digit == Option::None {
                first_digit = Option::Some(char.to_string());
            }

            last_digit = Option::Some(char.to_string());
        }
    }

    if first_digit == Option::None && last_digit == Option::None {
        return 0;
    }

    let result_digit = format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap();

    return result_digit;
}