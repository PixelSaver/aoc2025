use crate::AdventDay;

/// Library implementation for Day04.
///
/// Extracted from the binary so it can be invoked directly in tests.
#[derive(Debug, Default)]
pub struct Day06;

impl Day06 {
}

impl AdventDay for Day06 {
    fn part1(&self, input: &str) -> String {
        let mut grand_total : i64 = 0;
        
        let lines = input.lines().collect::<Vec<&str>>();
        let mut rows = lines.iter().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        let ops = rows.pop().expect("No operations found!");
        
        let n_rows = rows.len();
        let n_cols = rows[0].len();
        
        for col in 0..n_cols {
            let op = ops[col]; // operator for this column
        
            match op {
                "*" => {
                    // example: multiply all values in this column
                    let mut acc : i64 = 1;
                    for row in 0..n_rows {
                        acc *= rows[row][col].parse::<i64>().unwrap();
                    }
                    grand_total += acc;
                }
        
                "+" => {
                    let mut acc : i64 = 0;
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
    
        // Operator row
        let ops_line = rows.last().unwrap();
        let data_rows = &rows[..rows.len() - 1];
        
        let row_count = data_rows.len();
    
        // Determine column width of padded rows
        let width = data_rows.iter().map(|r| r.len()).max().unwrap();
        
        // Right-pad numbers for vertical alignment
        let padded: Vec<String> = data_rows
            .iter()
            .map(|r| format!("{:>width$}", r, width = width))
            .collect();
        
        // Collect vertical digits column-by-column
        let mut col_strings = vec![String::new(); width];
        
        for row in &padded {
            for (i, ch) in row.chars().enumerate() {
                if ch.is_ascii_digit() {
                    col_strings[i].push(ch);
                }
            }
        }
        
        // Turn vertical strings into numbers (skip empty columns)
        let out: Vec<i64> = col_strings
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        
        // Extract operators (filtered of whitespace)
        let ops: Vec<char> = ops_line.chars().filter(|c| !c.is_whitespace()).collect();
        
        // Chunk numbers by row count
        let chunks: Vec<&[i64]> = out.chunks(row_count).collect();
        
        let mut grand_total: i64 = 0;
        
        // Process problems right-to-left
        for (chunk, op) in chunks.iter().rev().zip(ops.iter().rev()) {
            let value: i64 = match op {
                '*' => chunk.iter().product(),
                '+' => chunk.iter().sum(),
                _   => panic!("Unknown operator"),
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
