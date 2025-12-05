struct Dial {
    position: i32,
}

impl Dial {
    fn new() -> Self {
        Self { position: 50 }
    }

    fn turn(&mut self, amount: i32) -> i32 {
        let new_position = (self.position + amount).rem_euclid(100);
        self.position = new_position;
        return new_position;
    }
}

fn main() {
    let input = include_str!("aoc2025day01_input.txt");
    let mut dial = Dial::new();
    let mut password = 0;
    for line in input.lines() {
        let val: i32 = line[1..].parse().unwrap();
        let direction = if "L" == &line[0..1] { -1 } else { 1 };
        for _ in 0..val {
            if dial.turn(direction) == 0 {
                password += 1;
            }
        }
    }
    println!("{:?}", password);
}
