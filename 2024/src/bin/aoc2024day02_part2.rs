use std::str::FromStr;

fn main() {
    let input: &str = include_str!("aoc2024day02_input.txt");
    let mut safe_count = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split_whitespace().map(str_to_i32).collect();
        let count = levels.len();
        if is_safe(&levels) {
            safe_count += 1;
        } else {
            'count: for i in 0..count {
                let mut cloned_levels = levels.clone();
                cloned_levels.remove(i);
                if is_safe(&cloned_levels) {
                    safe_count += 1;
                    break 'count;
                }
            }
        }
    }
    println!("safe_count: {safe_count}");
}

fn is_safe(levels: &[i32]) -> bool {
    let mut prev_delta: i32 = 0;
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
