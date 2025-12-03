use std::io;
use std::process::Command;
use std::time::Instant;

pub mod days;

/// The AdventDay trait - implement this for each day's solution.
///
/// Provides two small helpers in addition to `part1`/`part2`:
/// - `run` which reads from a file path then runs both parts
/// - `run_input` which runs both parts from an already-loaded string (useful for tests or stdin)
pub trait AdventDay {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;

    /// Read the given path and run parts 1 and 2 while printing timings.
    ///
    /// This method is more robust than the previous implementation:
    ///  - It first attempts to read the provided path.
    ///  - On failure it will attempt a few reasonable fallback locations (examples/*,
    ///    inputs/*examples.txt, inputs/*.example.txt) when the filename looks like `dayNN.txt`.
    ///  - If no file is found it prints helpful guidance before aborting.
    fn run(&self, input_path: &str) {
        // First try the supplied path.
        if let Ok(input) = std::fs::read_to_string(input_path) {
            self.run_input(&input);
            return;
        } else {
            eprintln!(
                "Failed to read input file '{}'. Will try common fallback locations...",
                input_path
            );
        }

        // If the supplied path looks like `.../dayNN.txt`, try common example locations.
        if let Some(base_name) = std::path::Path::new(input_path)
            .file_name()
            .and_then(|s| s.to_str())
        {
            if base_name.starts_with("day") && base_name.ends_with(".txt") {
                let stem = &base_name[..base_name.len() - 4]; // strip ".txt"
                let candidates = [
                    format!("examples/{}.txt", &stem),       // examples/dayNN.txt
                    format!("inputs/{}examples.txt", &stem), // inputs/dayNNexamples.txt
                    format!("inputs/{}.example.txt", &stem), // inputs/dayNN.example.txt
                ];

                for cand in &candidates {
                    if std::path::Path::new(cand).exists() {
                        eprintln!("Using fallback input file: {}", cand);
                        let input = std::fs::read_to_string(cand)
                            .unwrap_or_else(|e| panic!("Failed to read fallback {}: {}", cand, e));
                        self.run_input(&input);
                        return;
                    }
                }
            }
        }

        // No fallback found — print final guidance and abort.
        eprintln!(
            "No input found for '{}'.\n\
             Put the day's input at '{}' or place an example at 'examples/' or 'inputs/'.\n\
             You can also pass an explicit path to the binary, or run with --stdin and pipe data in.",
            input_path, input_path
        );
        std::process::exit(1);
    }

    /// Run parts 1 and 2 for an input already loaded into memory.
    fn run_input(&self, input: &str) {
        let start = Instant::now();
        let p1 = self.part1(input);
        println!("Part 1: {} ({:?})", p1, start.elapsed().as_millis());

        let start = Instant::now();
        let p2 = self.part2(input);
        println!("Part 2: {} ({:?})", p2, start.elapsed().as_millis());
    }
}

/// Returns the input filepath for any given day (primary input).
pub fn input_for(day: u32) -> String {
    format!("inputs/day{:02}.txt", day)
}

/// Returns an example input filepath for any given day.
/// Useful with a `--example` flag in day binaries.
pub fn example_for(day: u32) -> String {
    // Prefer an `examples/` directory by default
    format!("examples/day{:02}.txt", day)
}

/// Find an existing example file for the given day by trying several common locations.
/// Returns the first candidate path that exists, or `None` if none were found.
///
/// Candidate order:
///  - `examples/dayNN.txt`
///  - `inputs/dayNNexamples.txt`  (matches the project's current naming if used)
///  - `inputs/dayNN.example.txt`
pub fn find_existing_example(day: u32) -> Option<String> {
    let candidates = [
        format!("examples/day{:02}.txt", day),
        format!("inputs/day{:02}examples.txt", day),
        format!("inputs/day{:02}.example.txt", day),
    ];

    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return Some(c.clone());
        }
    }
    None
}

/// Helper that returns both canonical and example variants for a day.
/// Useful if you want to offer both options in a runner.
pub fn input_variants(day: u32) -> (String, String) {
    (input_for(day), example_for(day))
}

/// Open a file or URL with the system's default application.
///
/// - On Windows uses `cmd /C start`
///
/// Returns a std::io::Result so callers can report any failures.
pub fn open(path: &str) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        // `start` is a shell builtin; run via `cmd /C start`
        Command::new("cmd")
            .args(&["/C", "start", "", path])
            .spawn()
            .map(|_| ())
    }
}

/// Convenience macro to generate a standard `main` for day binaries.
///
/// Usage (in a binary like `src/bin/day01.rs`):
/// - Keep or add `include!("../infer_day.rs");` at the top of the file (it provides the `infer_day!()` macro).
/// - Replace the hand-written `main` with:
///     `aoc2025::advent_main!(Day01);`
///
/// The generated main supports these command-line behaviours:
/// - No args: read inputs/dayXX.txt and run.
/// - Provide a single `.txt` path: read that path and run.
/// - `--example` or `-e`: use examples/dayXX.txt instead of inputs/dayXX.txt.
/// - `--stdin`: read the input from stdin instead of a file.
/// - `--open`: open the input file with the system default app and exit.
/// You can combine `--example` with `--open`.
#[macro_export]
macro_rules! advent_main {
    ($day_ty:ident) => {
        fn main() {
            // Expect the local crate to provide `infer_day!()` (see infer_day.rs)
            let args: Vec<String> = std::env::args().collect();
            // infer the day number from the filename at compile time
            let day_num: u32 = infer_day!();

            let mut use_example = false;
            let mut open_flag = false;
            let mut use_stdin = false;
            let mut explicit_path: Option<String> = None;

            for a in &args[1..] {
                match a.as_str() {
                    "--example" | "-e" => use_example = true,
                    "--open" => open_flag = true,
                    "--stdin" => use_stdin = true,
                    other => {
                        // If a path to a .txt file is provided, prefer it
                        if other.ends_with(".txt") || other.contains(std::path::MAIN_SEPARATOR) {
                            explicit_path = Some(other.to_string());
                        }
                    }
                }
            }

            let path = if let Some(p) = explicit_path {
                p
            } else if use_example {
                match aoc2025::find_existing_example(day_num) {
                    Some(p) => p,
                    None => {
                        eprintln!(
                            "--example requested but no example file found. Searched: examples/day{:02}.txt, inputs/day{:02}examples.txt, inputs/day{:02}.example.txt",
                            day_num, day_num, day_num
                        );
                        std::process::exit(1);
                    }
                }
            } else {
                aoc2025::input_for(day_num)
            };

            if open_flag {
                match aoc2025::open(&path) {
                    Ok(_) => {
                        /* spawned opener; exit */
                        return;
                    }
                    Err(e) => {
                        eprintln!("Failed to open {}: {}", path, e);
                        // fallthrough to try reading the file locally
                    }
                }
            }

            let input = if use_stdin {
                std::io::read_to_string(std::io::stdin()).expect("Failed to read from stdin")
            } else {
                std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
            };

            // Day type is expected to be a unit-like struct (e.g. `struct Day01;`)
            let day = $day_ty;
            day.run_input(&input);
        }
    };
}
