use aoc2025::AdventDay;
use aoc2025::input_for;

include!("../infer_day.rs");

struct Day01;

impl AdventDay for Day01 {
    fn part1(&self, input: &str) -> String {
        // Boilerplate code
        input.lines().count().to_string()
    }
    fn part2(&self, input: &str) -> String {
        input.lines().count().to_string()
    }
}

fn main() {
    let day = infer_day!();

    Day01.run(&input_for(day));
}
