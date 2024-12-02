use std::str::FromStr;

fn main() {
    let input = include_str!("aoc2024day02_input.txt");
    let mut safe_count = 0;
    for line in input.lines() {
        if is_safe(line) {
            safe_count += 1;
        }
    }
    println!("safe_count: {safe_count}");
}

fn is_safe(line: &str) -> bool {
    let mut prev_delta: i32 = 0;
    let levels: Vec<i32> = line.split_whitespace().map(str_to_i32).collect();
    for levels in levels.windows(2) {
        let delta = levels[0] - levels[1];
        if !(delta.abs() >= 1 && delta.abs() <= 3) {
            return false;
        }
        if prev_delta != 0 && delta.signum() != prev_delta.signum() {
            return false;
        }
        prev_delta = delta;
    }
    true
}

fn str_to_i32(a: &str) -> i32 {
    i32::from_str(a).unwrap()
}
