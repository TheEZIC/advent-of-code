use std::fs;
use std::collections::HashMap;

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
    let words_to_digit = HashMap::<String, char>::from([
        (String::from("one"), '1'),
        (String::from("two"), '2'),
        (String::from("three"), '3'),
        (String::from("four"), '4'),
        (String::from("five"), '5'),
        (String::from("six"), '6'),
        (String::from("seven"), '7'),
        (String::from("eight"), '8'),
        (String::from("nine"), '9'),
    ]);

    let word_digits: Vec<String> = words_to_digit.clone().into_keys().collect();
    let digit_parts: Vec<char> = words_to_digit.clone().into_values().collect();

    let mut first_digit: Option<String> = Option::None;
    let mut last_digit: Option<String> = Option::None;

    let chars: Vec<char> = str.chars().collect();

    for (i, char) in chars.iter().enumerate() {
        let is_digit_part = digit_parts.contains(&char);

        let mut current_digit: Option<String> = Option::None;

        if is_digit_part {
            current_digit = Option::Some(char.to_string());
        } else {
            let mut current_word_digits: Vec<String> = word_digits.clone();
            let mut char_index: usize = 0;

            while current_word_digits.len() > 0 {
                if i + char_index > chars.len() - 1 {
                    break;
                }

                let word_char = chars[i + char_index];

                current_word_digits = current_word_digits
                    .into_iter()
                    .filter(|el| el.chars().nth(char_index).unwrap() == word_char)
                    .collect();

                let word_digits_left = current_word_digits.len();
                let current_word = &str.clone()[i..(i + char_index + 1)];
                let is_digit_word = word_digits_left == 1 && current_word == current_word_digits[0];

                if is_digit_word {
                    let k = &current_word_digits[0];
                    let c = words_to_digit.get(k).unwrap().to_string();
                    current_digit = Option::Some(c);
                    break;
                }

                char_index += 1;
            }
        }

        if current_digit != Option::None {
            if first_digit == Option::None {
                first_digit = Option::Some(current_digit.clone().unwrap().to_string());
            }

            last_digit = Option::Some(current_digit.clone().unwrap().to_string());
        }
    }

    if first_digit == Option::None && last_digit == Option::None {
        return 0;
    }

    let result_digit = format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap();

    return result_digit;
}