use std::time::Instant;

pub trait AdventDay {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;

    fn run(&self, input_path: &str) {
        let input = std::fs::read_to_string(input_path).expect("Failed to read input file");

        let start = Instant::now();
        let p1 = self.part1(&input);
        println!("Part 1: {} ({:?})", p1, start.elapsed().as_millis());

        let start = Instant::now();
        let p2 = self.part2(&input);
        println!("Part 2: {} ({:?})", p2, start.elapsed().as_millis());
    }
}

/// Returns the input filepath for any given day
pub fn input_for(day: u32) -> String {
    format!("inputs/day{:02}.txt", day)
}
