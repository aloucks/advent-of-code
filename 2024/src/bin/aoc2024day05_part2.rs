use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    let input = include_str!("aoc2024day05_input.txt");

    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::default();
    let mut updates: Vec<Vec<i64>> = Vec::default();

    let mut is_reading_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            is_reading_rules = false;
            continue;
        }
        if is_reading_rules {
            let mut parts = line.split("|").map(str_to_i64);
            let page_before = parts.next().unwrap();
            let page_after = parts.next().unwrap();
            rules.entry(page_before).or_default().insert(page_after);
        } else {
            updates.push(line.split(",").map(str_to_i64).collect());
        }
    }

    let mut total = 0;

    for update in updates.iter() {
        if !is_update_valid(update, &rules) {
            let mut update = update.clone();
            update.sort_by(|a, b| {
                if let Some(pages_must_be_after) = rules.get(&b) {
                    if pages_must_be_after.contains(&a) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Equal
                }
            });
            total += update[update.len() / 2];
        }
    }
    println!("total: {}", total);
}

fn is_update_valid(update: &[i64], rules: &HashMap<i64, HashSet<i64>>) -> bool {
    let mut is_valid = true;
    let mut pages = HashSet::new();
    for page in update.iter() {
        if let Some(pages_must_be_after) = rules.get(&page) {
            if pages.intersection(&pages_must_be_after).next().is_some() {
                is_valid = false;
                break;
            }
        }
        pages.insert(*page);
    }
    is_valid
}

fn str_to_i64(a: &str) -> i64 {
    a.parse().unwrap()
}
