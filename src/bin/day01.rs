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
    /// # Test
    /// ```rust
    /// let result = my_crate::add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    fn part1(&self, input: &str) -> String {
        let mut counter : u32 = 0;
        let mut dial : i32 = 50;

        for line in input.lines() {
            let mut chars = line.chars();
            let first_char = if let Some(c) = chars.next() {
                c // consume the first character, removed from chars
            } else {
                continue; // Skip empty lines
            };
            let dir = Direction::from_char(first_char);
            // Make the rest a string and then i32
            let number : i32 = chars.collect::<String>().parse().unwrap();
            let addendum = match dir {
                Direction::Left => -number,
                Direction::Right => number,
            };
            dial += addendum;
            dial = dial.rem_euclid(100);
            counter += if dial == 0 { 1 } else { 0 };
        }
        // Last line is same as return
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
