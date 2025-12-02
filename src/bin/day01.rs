use aoc2025::AdventDay;
use aoc2025::input_for;
use std::env;

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
        let mut counter: u32 = 0;
        let mut dial: i32 = 50;

        for line in input.lines() {
            let mut chars = line.chars();
            let first_char = if let Some(c) = chars.next() {
                c // consume the first character, removed from chars
            } else {
                continue; // Skip empty lines
            };
            let dir = Direction::from_char(first_char);
            // Make the rest a string and then i32
            let number: i32 = chars.collect::<String>().parse().unwrap();
            dial += match dir {
                Direction::Left => -number,
                Direction::Right => number,
            };
            dial = dial.rem_euclid(100);
            counter += if dial == 0 { 1 } else { 0 };
        }
        // Last line is same as return
        counter.to_string()
    }
    fn part2(&self, input: &str) -> String {
        let mut counter: u32 = 0;
        let mut dial: i32 = 50;

        for line in input.lines() {
            let mut chars = line.chars();
            let first_char = if let Some(c) = chars.next() {
                c // consume the first character, removed from chars
            } else {
                continue; // Skip empty lines
            };
            let dir = Direction::from_char(first_char);
            // Make the rest a string and then i32
            let number: i32 = chars.collect::<String>().parse().unwrap();
            let addendum = match dir {
                Direction::Left => -number,
                Direction::Right => number,
            };
            let curr_dial = dial;
            let zero_passes = self.passed_zero(&mut dial, addendum);
            println!(
                "Dial: {}   Addendum: {}   passed_zer: {}",
                curr_dial, addendum, zero_passes
            );
            counter += zero_passes;
            counter += if dial == 0 { 1 } else { 0 };
        }

        counter.to_string()
    }
}

impl Day01 {
    /// Adds addendum to dial
    /// Also returns a bool as to whether it passed 0
    fn passed_zero(&self, dial: &mut i32, addendum: i32) -> u32 {
        *dial += addendum;
        let before_wrap = *dial;
        // while *dial < 0 {
        //     *dial += 100;
        //     if *dial != 0 {
        //         zero_passes += 1;
        //     }
        // }
        // while *dial > 99 {
        //     *dial -= 100;
        //     if *dial != 0 {
        //         zero_passes += 1;
        //     }
        // }
        *dial = (*dial).rem_euclid(100);
        let zero_passes = if before_wrap == 0 {
            0
        } else if before_wrap > 0 {
            (before_wrap.div_euclid(100)) as u32
        } else {
            ((-before_wrap).div_euclid(100) + 1) as u32
        };
        zero_passes
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = infer_day!();

    if args.len() > 1 {
        Day01.run(&args[1])
    } else {
        Day01.run(&input_for(day));
    }
}
