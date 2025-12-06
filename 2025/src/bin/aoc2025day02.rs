fn main() {
    let input = include_str!("aoc2025day02_input.txt");
    let mut password = 0;
    for line in input.split(',') {
        let mut chunks = line.split('-');
        let num1: u64 = chunks.next().unwrap().parse().expect("num1");
        let num2: u64 = chunks.next().unwrap().parse().expect("num2");
        for value in num1..=num2 {
            let text = value.to_string();
            let len = text.len();
            if len % 2 == 0 {
                let half1 = &text[0..len / 2];
                let half2 = &text[len / 2..];
                if half1 == half2 {
                    password += value;
                }
            }
        }
    }
    println!("password: {password}");
}
