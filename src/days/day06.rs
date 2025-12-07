use crate::AdventDay;

/// Library implementation for Day04.
///
/// Extracted from the binary so it can be invoked directly in tests.
#[derive(Debug, Default)]
pub struct Day06;

impl Day06 {}

impl AdventDay for Day06 {
    fn part1(&self, input: &str) -> String {
        let mut grand_total: i64 = 0;

        let lines = input.lines().collect::<Vec<&str>>();
        let mut rows = lines
            .iter()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let ops = rows.pop().expect("No operations found!");

        let n_rows = rows.len();
        let n_cols = rows[0].len();

        for col in 0..n_cols {
            let op = ops[col]; // operator for this column

            match op {
                "*" => {
                    // example: multiply all values in this column
                    let mut acc: i64 = 1;
                    for row in 0..n_rows {
                        acc *= rows[row][col].parse::<i64>().unwrap();
                    }
                    grand_total += acc;
                }

                "+" => {
                    let mut acc: i64 = 0;
                    for row in 0..n_rows {
                        acc += rows[row][col].parse::<i64>().unwrap();
                    }
                    grand_total += acc;
                }

                _ => continue,
            }
        }
        grand_total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rows: Vec<&str> = input.lines().collect();

        // The operator row is the last line.
        let ops_line = rows.last().unwrap();
        // Data rows are all lines before the last one.
        let data_rows = &rows[..rows.len() - 1];

        // Determine the maximum column width for padding.
        let width = data_rows.iter().map(|r| r.len()).max().unwrap_or(0);

        // Right-pad numbers/rows for vertical alignment (important for alignment).
        let padded: Vec<String> = data_rows
            .iter()
            .map(|r| format!("{:>width$}", r, width = width))
            .collect();

        // Collect vertical digits column-by-column into strings.
        let mut col_strings = vec![String::new(); width];

        for row in &padded {
            for (i, ch) in row.chars().enumerate() {
                // A number column contains digits; a separator column is all spaces.
                if ch.is_ascii_digit() {
                    col_strings[i].push(ch);
                }
            }
        }

        // --- CRITICAL FIX: Group columns into problems using the empty strings as separators. ---

        let mut all_problems: Vec<Vec<i64>> = Vec::new();
        let mut current_problem_numbers: Vec<i64> = Vec::new();

        for col_str in col_strings.into_iter() {
            if col_str.is_empty() {
                if !current_problem_numbers.is_empty() {
                    current_problem_numbers.reverse();
                    all_problems.push(current_problem_numbers);
                    current_problem_numbers = Vec::new();
                }
            } else {
                // Found a vertical number column. Parse and add to the current problem list.
                let number = col_str.parse::<i64>().unwrap();
                current_problem_numbers.push(number);
            }
        }

        // Handle the last problem if the input doesn't end with a separator column.
        if !current_problem_numbers.is_empty() {
            current_problem_numbers.reverse();
            all_problems.push(current_problem_numbers);
        }

        // Extract and clean operators.
        let ops: Vec<char> = ops_line.chars().filter(|c| !c.is_whitespace()).collect();

        let mut grand_total: i64 = 0;

        // Process problems right-to-left (now they are correctly aligned).
        for (problem_numbers, op) in all_problems.iter().zip(ops.iter()) {
            let value: i64 = match op {
                '*' => problem_numbers.iter().product(),
                '+' => problem_numbers.iter().sum(),
                _ => panic!("Unknown operator"),
            };
            grand_total += value;
        }

        grand_total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day06;
    use crate::AdventDay;
    use crate::find_existing_example;
    use std::fs;

    #[test]
    fn day06_example_answers() {
        // Locate an example file using the project's helper (tries several locations).
        let path = find_existing_example(6).expect("No example input found for day06");
        let input = fs::read_to_string(&path).expect("Failed to read example file for day06");

        let day = Day06::default();

        // Expected answers provided by user:
        assert_eq!(day.part1(&input), "13");
        assert_eq!(day.part2(&input), "3121910778619");
    }
}
