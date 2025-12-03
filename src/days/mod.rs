//! Day modules for Advent of Code solutions.
//!
//! Place each day's library implementation in `src/days/dayNN.rs` and expose
//! the day type here so binaries and tests can import them as
//! `aoc2025::days::DayXX` or `aoc2025::days::dayXX::DayXX`.
pub mod day01;
pub mod day02;
pub mod day03;

// Re-export the most commonly used types for convenience.
pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
