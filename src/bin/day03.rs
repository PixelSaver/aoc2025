use aoc2025::AdventDay;
use aoc2025::input_for;
use std::env;

include!("../infer_day.rs");

struct Day03;

struct Bank {
    batteries: String,
}
impl Bank {
    fn new(s: &str) -> Self {
        Self {
            batteries: s.to_string(),
        }
    }
    fn max_joltage(&self, bank: &str) -> u64 {
        let chars: Vec<char> = bank.chars().collect();
        let n = chars.len();
    
        let mut max_val = 0;
    
        for i in 0..n {
            if let Some(d1) = chars[i].to_digit(10) {
                for j in i+1..n {
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
    fn max_joltage_k(&self, bank: &str, k: usize) -> u64 {
        let digits: Vec<u32> = bank
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
    
        let n = digits.len();
        let mut stack: Vec<u32> = Vec::new();
    
        for (i, &d) in digits.iter().enumerate() {
            // While the current digit is bigger than the last in stack,
            // and we can still fill enough digits to reach k
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
    
        // Convert the stack of digits into a number
        stack.iter().fold(0, |acc, &d| acc * 10 + d as u64)
    }
}

impl AdventDay for Day03 {
    fn part1(&self, input: &str) -> String {
        let mut counter: u64 = 0;
        // let mut banks : Vec<Bank> = Vec::new();
        for line in input.lines() {
            let bank = Bank::new(line);
            let max_j = bank.max_joltage(&bank.batteries);
            counter += max_j;
            // println!("Max j: {}", max_j);
        }
        counter.to_string()
    }
    fn part2(&self, input: &str) -> String {
        let mut counter: u64 = 0;
        // let mut banks : Vec<Bank> = Vec::new();
        for line in input.lines() {
            let bank = Bank::new(line);
            let max_j = bank.max_joltage_k(&bank.batteries, 12);
            counter += max_j;
            // println!("Max j: {}", max_j);
        }
        counter.to_string()
    }
}

impl Day03 {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = infer_day!();

    if args.len() > 1 {
        Day03.run(&args[1])
    } else {
        Day03.run(&input_for(day));
    }
}
