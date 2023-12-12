use std::fs;

pub fn read_input() -> Vec<String> {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    return lines;
}

pub fn calc_solutions_derivative(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut derivative = 1;

    for i in 0..times.len() {
        let solutions_count = calc_solutions_count(&times[i], &distances[i]);
        derivative *= solutions_count;
    }

    return derivative;
}

pub fn calc_solutions_count(time: &u64, distance: &u64) -> u64  {
    let time = *time;
    let distance = *distance;

    let d: f64 = (time * time - 4 * distance) as f64;
    let dsq = d.sqrt();

    let x1: f64 = (time as f64 + dsq) * 0.5;
    let x2: f64 = (time as f64 - dsq) * 0.5;

    let x1_has_precision = x1 != x1.floor();
    let x2_has_precision = x2 != x2.floor();

    let r = (x2.floor() - x1.floor()).abs() - f64::from(!x1_has_precision || !x2_has_precision);

    return r as u64;
}