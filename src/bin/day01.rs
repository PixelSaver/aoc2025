use aoc2025::AdventDay;

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
    Day01.run("inputs/day01.txt");
}