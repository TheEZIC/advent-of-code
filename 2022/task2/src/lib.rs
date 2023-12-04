use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub struct CubesBag {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl CubesBag {
    pub fn new() -> CubesBag {
        return CubesBag {
            red: 12,
            green: 13,
            blue: 14,
        }
    }
    
    pub fn empty() -> CubesBag {
        return CubesBag {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.red < 0 || self.green < 0 || self.blue < 0;
    }
}
