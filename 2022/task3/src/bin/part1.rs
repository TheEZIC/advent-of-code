use task3::{read_input, input_to_matrix, check_digit_char, set_skip_current};

fn main() {
    let lines = read_input();
    process_part1(lines);
}

fn process_part1(lines: Vec<String>) {
    if lines.len() == 0 {
        return;
    }

    let matrix = input_to_matrix(lines);

    let mut sum: u32 = 0;

    let min_line_index: usize = 0;
    let max_line_index: usize = matrix.len() - 1;

    let min_char_index: usize = 0;
    let max_char_index: usize = matrix[0].len() - 1;

    for (i, line) in matrix.iter().enumerate() {
        let mut current_digit = String::from("");
        let mut current_digit_index = 0;

        let is_first_line = i == min_line_index;
        let is_last_line = i == max_line_index;

        let mut skip_current_digit = true;

        for (j, char) in line.iter().enumerate() {
            let is_digit_part = check_digit_char(&char);

            let is_first_char = j == min_char_index;
            let is_last_char = j == max_char_index;

            if is_digit_part {
                if !is_first_char {
                    if !is_first_line {
                        set_skip_current(&mut skip_current_digit, &matrix[i - 1][j - 1]);
                    }

                    set_skip_current(&mut skip_current_digit, &matrix[i][j - 1]);

                    if !is_last_line {
                        set_skip_current(&mut skip_current_digit, &matrix[i + 1][j - 1]);
                    }
                }

                if !is_first_line {
                    set_skip_current(&mut skip_current_digit, &matrix[i - 1][j]);
                }

                if !is_last_line {
                    set_skip_current(&mut skip_current_digit, &matrix[i + 1][j]);
                }

                if !is_last_char {
                    if !is_first_line {
                        set_skip_current(&mut skip_current_digit, &matrix[i - 1][j + 1]);
                    }

                   set_skip_current(&mut skip_current_digit, &matrix[i][j + 1]);

                    if !is_last_line {
                        set_skip_current(&mut skip_current_digit, &matrix[i + 1][j + 1]);
                    }
                }

                current_digit.push(*char);
            }

            if (!is_digit_part || is_last_char) && current_digit.len() != 0 {
                if !skip_current_digit {
                    let digit = current_digit.parse::<u32>().unwrap();
                    sum += digit;
                }

                // println!("{} {}", skip_current_digit, current_digit);
                current_digit = String::from("");
                current_digit_index = 0;
            }

            if !is_digit_part {
                skip_current_digit = true;
            }
        }
    }

    println!("{}", sum);
}
