use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("aoc2025day05_input.txt");

    let mut ranges = Vec::new();
    let mut available = Vec::new();
    let mut loading_ranges = true;
    for line in input.lines() {
        if line.is_empty() {
            loading_ranges = false;
            continue;
        }

        if loading_ranges {
            ranges.push(FreshRange::new(&line));
        } else {
            available.push(u64::from_str_radix(line, 10).expect("id u64"));
        }
    }

    let mut fresh = Vec::new();

    for id in available.iter().copied() {
        for fr in ranges.iter() {
            if fr.is_fresh(id) {
                fresh.push(id);
                break;
            }
        }
    }

    println!("fresh.len: {}", fresh.len());
}

pub struct FreshRange {
    range: RangeInclusive<u64>,
}

impl FreshRange {
    pub fn new(line: &str) -> FreshRange {
        let mut chunks = line.split('-');
        let start = chunks
            .next()
            .expect("range start")
            .parse()
            .expect("range start u64");
        let end = chunks
            .next()
            .expect("range end")
            .parse()
            .expect("range end u64");
        FreshRange {
            range: RangeInclusive::new(start, end),
        }
    }

    fn is_fresh(&self, id: u64) -> bool {
        self.range.contains(&id)
    }
}
