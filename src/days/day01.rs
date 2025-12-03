use crate::AdventDay;

/// Day01 implementation extracted from the binary.
///
/// The struct is public so tests and the bin wrapper can instantiate it.
pub struct Day01;

#[derive(Debug, Clone, Copy)]
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
        }

        counter.to_string()
    }
}

impl Day01 {
    /// Adds addendum to dial and returns the number of times the dial passed zero.
    ///
    /// The dial is on range 0..100 and wraps. This computes how many times a crossing
    /// of 0 happened while moving `addendum` from the current `*dial`.
    pub fn passed_zero(&self, dial: &mut i32, addendum: i32) -> u32 {
        let start = *dial;
        let distance = addendum.abs();

        let mut count = 0;

        if addendum > 0 {
            // moving right (increasing)
            if start == 0 {
                // Starting at 0, we hit it again every 100 clicks
                count = (distance / 100) as u32;
            } else {
                let first_zero = 100 - start;
                if distance >= first_zero {
                    count = 1 + ((distance - first_zero) / 100) as u32;
                }
            }
        } else if addendum < 0 {
            // moving left (decreasing)
            if start == 0 {
                // Starting at 0, we hit it again every 100 clicks
                count = (distance / 100) as u32;
            } else {
                let first_zero = start;
                if distance >= first_zero {
                    count = 1 + ((distance - first_zero) / 100) as u32;
                }
            }
        }

        *dial = (start + addendum).rem_euclid(100);

        count
    }
}
