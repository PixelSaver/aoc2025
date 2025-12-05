use crate::AdventDay;
use std::collections::HashSet;

/// Library implementation for Day04.
///
/// Extracted from the binary so it can be invoked directly in tests.
#[derive(Debug, Default)]
pub struct Day05;

struct Range {
    min: i64,
    max: i64,
}
impl Range {
    fn new(minimum: i64, maximum: i64) -> Self {
        Range {
            min: minimum,
            max: maximum,
        }
    }
    fn is_within(&self, val: i64) -> bool {
        val >= self.min && val <= self.max
    }
}

impl Day05 {}

impl AdventDay for Day05 {
    fn part1(&self, input: &str) -> String {
        let input = input.replace("\r\n", "\n");
        let parts: Vec<&str> = input.split("\n\n").collect();
        if parts.len() < 2 {
            panic!("Input must contain two sections separated by a double newline!");
        }

        let tot_ranges: Vec<Range> = parts[0]
            .lines()
            .filter(|line| !line.trim().is_empty()) // skip empty lines
            .filter_map(|line| {
                let nums: Vec<i64> = line
                    .split('-')
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect();
                if nums.len() == 2 {
                    Some(Range::new(nums[0], nums[1]))
                } else {
                    eprintln!("Warning: skipping invalid range '{}'", line);
                    None
                }
            })
            .collect();

        let ids: Vec<i64> = parts[1]
            .lines()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        let count_fresh = ids
            .iter()
            .filter(|&&num| tot_ranges.iter().any(|range| range.is_within(num)))
            .count();

        count_fresh.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = input.replace("\r\n", "\n");
        let parts: Vec<&str> = input.split("\n\n").collect();
        if parts.len() < 2 {
            panic!("Input must contain two sections separated by a double newline!");
        }

        let mut ranges: Vec<Range> = parts[0]
            .lines()
            .filter(|line| !line.trim().is_empty()) // skip empty lines
            .filter_map(|line| {
                let nums: Vec<i64> = line
                    .split('-')
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect();
                if nums.len() == 2 {
                    Some(Range::new(nums[0], nums[1]))
                } else {
                    eprintln!("Warning: skipping invalid range '{}'", line);
                    None
                }
            })
            .collect();

        ranges.sort_by_key(|r| r.min);

        let mut merged: Vec<Range> = vec![];
        for r in ranges {
            if let Some(last) = merged.last_mut() {
                if r.min <= last.max + 1 {
                    last.max = last.max.max(r.max); // extend the last range
                    continue;
                }
            }
            merged.push(r);
        }

        // Count total IDs
        merged.iter().map(|r| (r.max - r.min + 1) as i64).sum::<i64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day05;
    use crate::AdventDay;
    use crate::find_existing_example;
    use std::fs;

    #[test]
    fn day05_example_answers() {
        // Locate an example file using the project's helper (tries several locations).
        let path = find_existing_example(5).expect("No example input found for day05");
        let input = fs::read_to_string(&path).expect("Failed to read example file for day05");

        let day = Day05::default();

        // Expected answers provided by user:
        assert_eq!(day.part1(&input), "13");
        assert_eq!(day.part2(&input), "3121910778619");
    }
}
