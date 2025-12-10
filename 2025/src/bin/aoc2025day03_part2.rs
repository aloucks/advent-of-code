fn main() {
    let input = include_str!("aoc2025day03_input.txt");
    let mut total_joltage = 0;
    for line in input.lines() {
        let joltage = parse_bank(line, 12);
        total_joltage += joltage;
    }
    println!("total_joltage: {total_joltage}");
}

fn to_digit_char(value: u64) -> char {
    char::from_digit(value as u32, 10).expect("from_digit")
}

fn parse_bank(line: &str, num_batteries_enabled: usize) -> u64 {
    let mut bank = Vec::new();
    for pos in 0..line.len() {
        let joltage: u64 = line[pos..pos + 1].parse().expect("parse");
        bank.push(joltage);
    }

    let mut joltage_string = String::new();

    for (pos, joltage) in bank.iter().copied().enumerate() {
        let mut take_it = joltage_string.len() < num_batteries_enabled;
        let bigger_joltage = bank[pos..].iter().copied().skip(1).find(|x| *x > joltage);
        if let Some(bigger_joltage) = bigger_joltage {
            let bigger_joltage_position = pos
                + bank
                    .iter()
                    .skip(pos)
                    .copied()
                    .position(|x| x == bigger_joltage)
                    .unwrap();
            let remaining_batteries_including_bigger_joltage = bank.len() - bigger_joltage_position;
            let needed_batteries = num_batteries_enabled - joltage_string.len();
            if needed_batteries <= remaining_batteries_including_bigger_joltage {
                take_it = false;
            }
        }

        if take_it {
            joltage_string.push(to_digit_char(joltage));
        }
    }

    return joltage_string.parse().unwrap();
}

#[test]
fn test_parse_bank_2() {
    assert_eq!(98, parse_bank("987654321111111", 2));
    assert_eq!(89, parse_bank("811111111111119", 2));
    assert_eq!(78, parse_bank("234234234234278", 2));
    assert_eq!(92, parse_bank("818181911112111", 2));
}

#[test]
fn test_parse_bank_12() {
    assert_eq!(987654321111, parse_bank("987654321111111", 12));
    assert_eq!(811111111119, parse_bank("811111111111119", 12));
    assert_eq!(434234234278, parse_bank("234234234234278", 12));
    assert_eq!(888911112111, parse_bank("818181911112111", 12));
}
