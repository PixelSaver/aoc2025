use crate::AdventDay;

/// Library implementation for Day03.
///
/// Extracted from the binary so it can be invoked directly in tests.
#[derive(Debug, Default)]
pub struct Day03;

struct Bank {
    batteries: String,
}

impl Bank {
    pub fn new(s: &str) -> Self {
        Self {
            batteries: s.to_string(),
        }
    }

    /// Compute the maximum two-digit joltage obtainable from the bank string.
    pub fn max_joltage(&self, bank: &str) -> u64 {
        let chars: Vec<char> = bank.chars().collect();
        let n = chars.len();

        let mut max_val = 0;

        for i in 0..n {
            if let Some(d1) = chars[i].to_digit(10) {
                for j in i + 1..n {
                    if let Some(d2) = chars[j].to_digit(10) {
                        let val = d1 * 10 + d2;
                        if val > max_val {
                            max_val = val;
                        }
                    }
                }
            }
        }

        max_val as u64
    }

    /// Compute the maximum number obtainable by selecting `k` digits (in order)
    /// to form the largest possible k-digit number.
    pub fn max_joltage_k(&self, bank: &str, k: usize) -> u64 {
        let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        let n = digits.len();
        if k == 0 || n == 0 {
            return 0;
        }

        let mut stack: Vec<u32> = Vec::new();

        for (i, &d) in digits.iter().enumerate() {
            // While current digit is greater than last in stack and we can still
            // fill up to k digits from the remaining characters, pop.
            while let Some(&top) = stack.last() {
                if d > top && stack.len() + (n - i - 1) >= k {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.len() < k {
                stack.push(d);
            }
        }

        // Convert the stack to a number
        stack.iter().fold(0, |acc, &d| acc * 10 + d as u64)
    }
}

impl AdventDay for Day03 {
    fn part1(&self, input: &str) -> String {
        let mut counter: u64 = 0;
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let bank = Bank::new(line);
            let max_j = bank.max_joltage(&bank.batteries);
            counter += max_j;
        }
        counter.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut counter: u64 = 0;
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let bank = Bank::new(line);
            // original binary used k = 12
            let max_j = bank.max_joltage_k(&bank.batteries, 12);
            counter += max_j;
        }
        counter.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day03;
    use crate::AdventDay;
    use crate::find_existing_example;
    use std::fs;

    #[test]
    fn day03_example_answers() {
        // Locate an example file using the project's helper (tries several locations).
        let path = find_existing_example(3).expect("No example input found for day03");
        let input = fs::read_to_string(&path).expect("Failed to read example file for day03");

        let day = Day03::default();

        // Expected answers provided by user:
        assert_eq!(day.part1(&input), "357");
        assert_eq!(day.part2(&input), "3121910778619");
    }
}
