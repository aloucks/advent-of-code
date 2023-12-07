fn main() {
    let input = include_str!("aoc2015day01-input.txt");
    let floor = input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => 0,
    });
    println!("{}", floor);
}
