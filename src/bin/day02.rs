use aoc2025::AdventDay;
use aoc2025::input_for;
use std::env;

include!("../infer_day.rs");

struct Day02;

struct Range {
    lower: u64,
    upper: u64,
}
impl Range {
    fn from_str(s: &str) -> Self {
        let mut str_vec = s.split('-');
        let first = str_vec.next().unwrap().parse().unwrap();
        let second = str_vec.next().unwrap().parse().unwrap();
        Range {
            lower: first,
            upper: second,
        }
    }
}

impl AdventDay for Day02 {
    fn part1(&self, input: &str) -> String {
        let mut counter: u64 = 0;
        let ranges: Vec<&str> = input.split(',').collect();
        for range in ranges {
            let curr_range = Range::from_str(range);
            counter += self.sum_of_invalid_ids(curr_range) as u64
        }
        counter.to_string()
    }
    fn part2(&self, _input: &str) -> String {
        let counter: u64 = 0;
        counter.to_string()
    }
}

impl Day02 {
    fn sum_of_invalid_ids(&self, range: Range) -> u64 {
        let mut counter: u64 = 0;
        for num in range.lower..=range.upper {
            if self.is_invalid_id(num) {
                counter += num;
            }
        }
        counter
    }
    fn is_invalid_id(&self, num: u64) -> bool {
        let mut s = num.to_string();
        let n = s.len();
        
        if n%2!=0 {
            s = format!("0{}", s);
            // Add 0s at the start to make it even
        }
        let pattern_len = n/2;
        if &s[0..pattern_len] == &s[pattern_len..n] {
            return true;
        }
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = infer_day!();

    if args.len() > 1 {
        Day02.run(&args[1])
    } else {
        Day02.run(&input_for(day));
    }
}
