use aoc2025::AdventDay;

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
            counter += self.sum_of_invalid_ids_p1(curr_range) as u64
        }
        counter.to_string()
    }
    fn part2(&self, input: &str) -> String {
        let mut counter: u64 = 0;
        let ranges: Vec<&str> = input.split(',').collect();
        for range in ranges {
            let curr_range = Range::from_str(range);
            counter += self.sum_of_invalid_ids_p2(curr_range) as u64
        }
        counter.to_string()
    }
}

impl Day02 {
    fn sum_of_invalid_ids_p1(&self, range: Range) -> u64 {
        let mut counter: u64 = 0;
        for num in range.lower..=range.upper {
            if self.is_invalid_id_p1(num) {
                counter += num;
            }
        }
        counter
    }
    fn sum_of_invalid_ids_p2(&self, range: Range) -> u64 {
        let mut counter: u64 = 0;
        for num in range.lower..=range.upper {
            if self.is_invalid_id_p2(num) {
                counter += num;
            }
        }
        counter
    }
    fn is_invalid_id_p1(&self, num: u64) -> bool {
        let s = num.to_string();

        self.check_repeat(&s, 2)
    }
    fn is_invalid_id_p2(&self, num: u64) -> bool {
        let s = num.to_string();
        let n = s.len() as u32;

        for repeat in 2..=n {
            if self.check_repeat(&s, repeat) {
                // println!("Invalid id: {}", num);
                return true;
            }
        }
        false
    }
    fn check_repeat(&self, str: &String, repeat: u32) -> bool {
        let n = str.chars().count() as u32;

        // If the string length is not divisible by repeat, cannot split evenly
        if n % repeat != 0 {
            return false;
        }

        let pattern_len = (n / repeat) as usize;
        let pat = &str[0..pattern_len];

        // Compare all chunks
        for i in 1..repeat {
            let start = (i * pattern_len as u32) as usize;
            let end = start + pattern_len;
            if &str[start..end] != pat {
                return false;
            }
        }
        // println!("Invalid: {}", str);
        true
    }
}

aoc2025::advent_main!(Day02);
