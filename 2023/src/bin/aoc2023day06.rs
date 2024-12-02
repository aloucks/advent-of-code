#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
struct Race {
    time: u64,
    dist: u64,
}

fn main() {
    let input = include_str!("aoc2023day06_input.txt");
    let mut lines = input.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let times: Vec<_> = line1
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|val| val.trim().parse::<u64>().unwrap())
        .collect();
    let dists: Vec<_> = line2
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|val| val.trim().parse::<u64>().unwrap())
        .collect();

    assert_eq!(times.len(), dists.len());

    let races = times
        .iter()
        .copied()
        .zip(dists.iter().copied())
        .map(|(time, dist)| Race { time, dist });

    for race in races {
        println!("{:?}", race);
    }
}
