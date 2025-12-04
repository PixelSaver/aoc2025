use aoc2025::AdventDay;
use aoc2025::{
    days::{Day01, Day02, Day03, Day04},
    find_existing_example,
};
use std::fs;

/// Smoke test for Day 01 example input.
/// Verifies that the example file can be found and both parts produce non-empty outputs.
#[test]
fn day01_example() {
    let example_path = find_existing_example(1).expect("No example found for day 1");
    let input = fs::read_to_string(&example_path).expect("Failed to read day1 example file");

    // Day01 is a unit-like struct
    let day = Day01;
    let p1 = day.part1(&input);
    let p2 = day.part2(&input);

    assert!(
        !p1.trim().is_empty(),
        "Day01 part1 returned empty for the example"
    );
    assert!(
        !p2.trim().is_empty(),
        "Day01 part2 returned empty for the example"
    );
}

/// Smoke test for Day 02 example input.
/// Verifies that the example file can be found and both parts produce non-empty outputs.
#[test]
fn day02_example() {
    let example_path = find_existing_example(2).expect("No example found for day 2");
    let input = fs::read_to_string(&example_path).expect("Failed to read day2 example file");

    // Day02 can be instantiated (unit-like / default)
    let day = Day02::default();
    let p1 = day.part1(&input);
    let p2 = day.part2(&input);

    assert!(
        !p1.trim().is_empty(),
        "Day02 part1 returned empty for the example"
    );
    assert!(
        !p2.trim().is_empty(),
        "Day02 part2 returned empty for the example"
    );
}

/// Functional test for Day 03 example input using the expected answers provided.
///
/// Expected answers (from user):
///  - Part 1: "357"
///  - Part 2: "3121910778619"
#[test]
fn day03_example_matches_expected() {
    let example_path = find_existing_example(3).expect("No example found for day 3");
    let input = fs::read_to_string(&example_path).expect("Failed to read day3 example file");

    let day = Day03::default();
    let p1 = day.part1(&input);
    let p2 = day.part2(&input);

    assert_eq!(
        p1, "357",
        "Day03 part1 did not match expected example answer"
    );
    assert_eq!(
        p2, "3121910778619",
        "Day03 part2 did not match expected example answer"
    );
}

#[test]
fn day04_example_matches_expected() {
    let example_path = find_existing_example(4).expect("No example found for day 4");
    let input = fs::read_to_string(&example_path).expect("Failed to read day4 example file");

    let day = Day04::default();
    let p1 = day.part1(&input);
    let p2 = day.part2(&input);

    assert_eq!(
        p1, "13",
        "Day04 part1 did not match expected example answer"
    );
    assert_eq!(
        p2, "3121910778619",
        "Day04 part2 did not match expected example answer"
    );
}