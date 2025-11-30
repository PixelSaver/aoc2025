
pub trait AdventDay {
    fn part1(&self, input:&str) -> String;
    fn part2(&self, input:&str) -> String;
    
    fn run(&self, input_path: &str) {
        let input = std::fs::read_to_string(input_path)
            .expect("Failed to read input file");
        
        println!("Part 1: {}", self.part1(&input)); 
        println!("Part 2: {}", self.part2(&input));
    }
}