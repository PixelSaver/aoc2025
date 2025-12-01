/// Infers the Advent of Code day number from the filename.
/// 
/// # Example
/// ```
/// // In src/bin/day05.rs
/// let day = infer_day!(); // returns 5
/// ```
#[macro_export] // for fully public macros
macro_rules! infer_day {
    () => {{
        // Extract "01" from "day01.rs"
        let file = file!(); // To read filepath from file
        let day = file.split("day").nth(1).unwrap().split(".rs").next().unwrap();
        day.parse::<u32>().unwrap()
    }};
}