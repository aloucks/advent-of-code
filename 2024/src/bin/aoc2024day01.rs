use std::str::FromStr;

fn main() {
    let input = include_str!("aoc2024day01_input.txt");
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        a.push(i32::from_str(parts.next().unwrap()).unwrap());
        b.push(i32::from_str(parts.next().unwrap()).unwrap());
    }
    a.sort();
    b.sort();

    let mut distance = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        distance += (a - b).abs();
    }
    println!("distance: {}", distance);
}
