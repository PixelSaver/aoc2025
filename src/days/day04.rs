use crate::AdventDay;

/// Library implementation for Day04.
///
/// Extracted from the binary so it can be invoked directly in tests.
#[derive(Debug, Default)]
pub struct Day04;

impl Day04 {
    fn clear_rolls(&self, grid:&mut Vec<Vec<char>>) -> i32 {
        let mut out = grid.to_owned();
        let rows = grid.len();
        let cols = grid[0].len();
        
        // 8 directions
        let dirs = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];
        
        let mut counter = 0;
        
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }
        
                let mut count = 0;
        
                for (dr, dc) in dirs {
                    let new_r = r as isize + dr;
                    let new_c = c as isize + dc;
        
                    if new_r >= 0 && new_r < rows as isize &&
                       new_c >= 0 && new_c < cols as isize &&
                       grid[new_r as usize][new_c as usize] == '@'
                    {
                        count += 1;
                    }
                }
        
                if count < 4 {
                    out[r][c] = '.';
                    counter += 1;
                }
            }
        }
        *grid = out;
        return counter;
    }
}

impl AdventDay for Day04 {
    fn part1(&self, input: &str) -> String {
        // Read grid into Vec<Vec<char>>
        let mut grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        if grid.is_empty() {
            return "0".to_string();
        }
        let num = self.clear_rolls(&mut grid);
        num.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        if grid.is_empty() {
            return "0".to_string();
        }
        let mut counter = 0;
        while true {
            let num = self.clear_rolls(&mut grid);
            counter += num;
            if num == 0 {
                break;
            }
        }
        counter.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day04;
    use crate::AdventDay;
    use crate::find_existing_example;
    use std::fs;

    #[test]
    fn day04_example_answers() {
        // Locate an example file using the project's helper (tries several locations).
        let path = find_existing_example(4).expect("No example input found for day04");
        let input = fs::read_to_string(&path).expect("Failed to read example file for day04");

        let day = Day04::default();

        // Expected answers provided by user:
        assert_eq!(day.part1(&input), "13");
        assert_eq!(day.part2(&input), "3121910778619");
    }
}
