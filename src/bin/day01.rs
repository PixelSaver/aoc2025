use aoc2025::AdventDay;
use aoc2025::input_for;

include!("../infer_day.rs");

struct Day01;

enum Direction {
    Left,
    Right,
}
impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction character: {}", c),
        }
    }
}

impl AdventDay for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut counter : u32 = 0;
        let mut dial : i32 = 50;

        for line in input.lines() {
            let mut chars = line.chars();
            let first_char = match chars.next() { // Consume the first character
                Some(c) => c,
                None => continue, // Skip empty lines
            };
            let dir = Direction::from_char(first_char);
            // Make the rest a string and then i32
            let number : i32 = chars.collect::<String>().parse().unwrap();
            match dir {
                Direction::Left => {
                    dial -= number;
                },
                Direction::Right => {
                    dial += number;
                }
            }
            let wrapped = ((dial % 100) + 100) % 100;
            if wrapped == 0 {
                counter += 1;
            }
            dial = wrapped;
        }
        
        counter.to_string()
    }
    fn part2(&self, input: &str) -> String {
        input.lines().count().to_string()
    }
}

fn main() {
    let day = infer_day!();

    Day01.run(&input_for(day));
}
