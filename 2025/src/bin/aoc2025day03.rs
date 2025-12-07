fn main() {
    let input = include_str!("aoc2025day03_input.txt");
    let mut total_joltage = 0;
    for line in input.lines() {
        let joltage = parse_bank(line);
        total_joltage += joltage;
    }
    println!("total_joltage: {total_joltage}");
}

fn parse_bank(line: &str) -> u64 {
    let mut bank = Vec::new();
    for pos in 0..line.len() {
        let joltage: u64 = line[pos..pos + 1].parse().unwrap();
        bank.push(joltage);
    }
    let max_joltage = bank.iter().copied().max().unwrap();
    let max_joltage_position = bank.iter().position(|j| *j == max_joltage).unwrap();

    let (new_bank_range, end) = if max_joltage_position == bank.len() - 1 {
        (&bank[0..max_joltage_position], true)
    } else {
        (&bank[1 + max_joltage_position..], false)
    };
    let next_max_joltage = new_bank_range.iter().copied().max().unwrap();
    let pair = if end {
        [next_max_joltage, max_joltage]
    } else {
        [max_joltage, next_max_joltage]
    };
    let joltage_string = format!("{}{}", pair[0], pair[1]);
    return joltage_string.parse().unwrap();
}

#[test]
fn test_parse_bank() {
    assert_eq!(98, parse_bank("987654321111111"));
    assert_eq!(89, parse_bank("811111111111119"));
    assert_eq!(78, parse_bank("234234234234278"));
    assert_eq!(92, parse_bank("818181911112111"));
}
