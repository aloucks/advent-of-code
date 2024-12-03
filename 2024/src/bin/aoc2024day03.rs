use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = include_str!("aoc2024day03_input.txt");
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    let mut total = 0;
    for capture in re.captures_iter(input) {
        let a = capture.get(1).map(|m| str_to_i32(m.as_str())).unwrap();
        let b = capture.get(2).map(|m| str_to_i32(m.as_str())).unwrap();
        total += a * b;
    }
    println!("total: {total}");
}

fn str_to_i32(a: &str) -> i32 {
    i32::from_str(a).unwrap()
}
