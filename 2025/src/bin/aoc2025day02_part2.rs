fn main() {
    let input = include_str!("aoc2025day02_input.txt");
    let mut password = 0;
    for line in input.split(',') {
        let mut chunks = line.split('-');
        let num1: u64 = chunks.next().unwrap().parse().expect("num1");
        let num2: u64 = chunks.next().unwrap().parse().expect("num2");
        for value in num1..=num2 {
            let text = value.to_string();
            for key_len in 1..=(text.len() / 2) {
                let key = &text[0..key_len];
                if is_repeats(key, &text) {
                    password += value;
                    break;
                }
            }
        }
    }
    println!("password: {password}");
}

fn is_repeats(key: &str, text: &str) -> bool {
    if text.len() % key.len() == 0 && text.len() > key.len() {
        for pos in (0..text.len()).step_by(key.len()) {
            let check = &text[pos..pos + key.len()];
            if key != check {
                return false;
            }
        }
        return true;
    }
    return false;
}

#[test]
fn test_is_repeats() {
    assert_eq!(false, is_repeats("1", "12"));
    assert_eq!(false, is_repeats("12", "12"));
    assert_eq!(true, is_repeats("1", "11"));
    assert_eq!(true, is_repeats("123", "123123"));
    assert_eq!(true, is_repeats("123", "123123123"));
    assert_eq!(false, is_repeats("123", "123123124"));
}
