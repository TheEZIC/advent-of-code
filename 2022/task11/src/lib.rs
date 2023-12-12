use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn solve(lines: &Vec<String>, multiplier: u64) {
    let matrix = generate_matrix(lines);
    let empty_lines = EmptyLines::from_matrix(&matrix);
    let coordinates = collect_coordinates(&matrix);
    let result: u64 = coordinates
        .iter()
        .enumerate()
        .map(|(i, c)| find_sum_coord(&coordinates[(i + 1)..].to_vec(), &c, &empty_lines, multiplier))
        .sum();

    println!("{}", result);
}

fn find_sum_coord(coordinates: &Vec<Vector2D>, coordinate: &Vector2D, empty_lines: &EmptyLines, multiplier: u64) -> u64 {
    let (x1, y1) = coordinate;

    let m: u64 = coordinates
        .iter()
        .map(|(x2, y2)| {
            let x_range = x1.min(x2)..x1.max(x2);
            let y_range = y1.min(y2)..y1.max(y2);

            let mut x = x_range.end - x_range.start;
            let mut y = y_range.end - y_range.start;

            for empty_x in empty_lines.x.iter() {
                if x_range.contains(&empty_x) {
                    x += multiplier - 1;
                }
            }

            for empty_y in empty_lines.y.iter() {
                if y_range.contains(&empty_y) {
                    y += multiplier - 1;
                }
            }

            return x + y;
        })
        .sum();

    return m;
}

fn generate_matrix(lines: &Vec<String>) -> Vec<Vec<char>> {
    return lines
        .iter()
        .map(|str| str.chars().collect())
        .collect::<Vec<Vec<char>>>();
}

fn collect_coordinates(matrix: &Vec<Vec<char>>) -> Vec<Vector2D> {
    return matrix
        .iter()
        .enumerate()
        .map(|(i, str)| {
            str.iter()
                .enumerate()
                .map(|(j, c)| {
                    if *c == '#' {
                        Some((i as u64, j as u64))
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Vector2D>>()
        })
        .flatten()
        .collect::<Vec<Vector2D>>();
}

struct EmptyLines {
    x: Vec<u64>,
    y: Vec<u64>,
}

impl EmptyLines {
    fn new() -> EmptyLines {
        return EmptyLines {
            x: Vec::new(),
            y: Vec::new(),
        };
    }

    fn from_matrix(matrix: &Vec<Vec<char>>) -> EmptyLines {
        let mut empty_lines = EmptyLines::new();

        if matrix.len() == 0 {
            return empty_lines;
        }

        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < matrix.len() && j < matrix[0].len() {
            let is_row_empty = matrix[i].iter().all(|el| *el == '.');
            let is_col_empty = matrix
                .iter()
                .enumerate()
                .map(|(i, _)| matrix[i][j])
                .all(|el| el == '.');

            if is_row_empty {
                empty_lines.x.push(i as u64);
            }

            if is_col_empty {
                empty_lines.y.push(j as u64);
            }

            i += 1;
            j += 1;
        }

        return empty_lines;
    }
}

type Vector2D = (u64, u64);