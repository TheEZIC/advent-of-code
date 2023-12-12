use std::collections::HashMap;
use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn read_direction_line(line: &String) -> Vec<char> {
    return line.chars().collect::<Vec<char>>();
}

pub fn read_path_lines(lines: &Vec<String>) -> RouteMap {
    let mut map: RouteMap = HashMap::new();

    for line in lines {
        let (source, paths_chunk) = line.split_once("=").unwrap();
        let source = source.trim().to_string();

        let paths_chunk = paths_chunk.replace("(", "");
        let paths_chunk = paths_chunk.replace(")", "");

        let (left, right) = paths_chunk.split_once(",").unwrap();
        let left = left.trim().to_string();
        let right = right.trim().to_string();

        map.insert(source, (left, right));
    }

    return map;
}

pub type Route = (String, String);

pub type RouteMap = HashMap<String, Route>;