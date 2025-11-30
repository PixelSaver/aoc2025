use aoc2025::AdventDay;
use std::env;

struct Day01;

impl AdventDay for Day01 {
    fn part1(&self, input:&str) -> String {
        // Boilerplate code
        input.lines().count().to_string()
    }
    fn part2(&self, input:&str) -> String {
        input.lines().count().to_string()
    }
}

fn main() {
    let input_file = env::args().nth(1).unwrap_or("inputs/day01.txt".into());
    Day01.run(&input_file);
}