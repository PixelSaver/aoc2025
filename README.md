# Advent of Code 2025 - PixelSaver

Heyo!!

## Running the scripts
You can run any day binary with cargo; the crate provides a small helper macro and a few convenient CLI flags to make this easy.

- Basic run (uses `inputs/dayNN.txt`):
```zsh
cargo run --bin day01
```

- Provide an explicit input file path:
```zsh
cargo run --bin day01 -- inputs/day02.txt
```

- Use the example input for the day (uses `examples/dayNN.txt`):
```zsh
cargo run --bin day01 -- --example
```

- Read the input from stdin (useful for piping):
```zsh
cargo run --bin day01 -- --stdin < inputs/day01.txt
```

- Open the resolved input file with your OS default application (best-effort):
```zsh
cargo run --bin day01 -- --open
```

Notes:
- Example files live in `examples/` by convention; place `examples/day01.txt`, etc. there if you want `--example` to work.
- Day binaries in `src/bin/` should include `include!("../infer_day.rs");` and expose a unit-like struct (for example `struct Day01;`). Then you can replace the handwritten `main` with:
```rust
aoc2025::advent_main!(Day01);
```
This macro wires up the flags above and runs both parts, printing timings.

## Testing

This repository includes example-driven tests so you can assert known correct answers for the example inputs.

- To run all tests:
```bash
cargo test
```

- To run only the integration test file that checks example outputs:
```bash
cargo test --test examples
```

- To run a single test by name (useful when debugging):
```bash
cargo test day03_example_matches_expected -- --nocapture
```

Where tests live:
- Integration tests: `tests/examples.rs` (contains example-driven checks for days)
- Day library implementations: `src/days/dayNN.rs` (each day exposes a `DayNN` type implementing `AdventDay`)

How tests locate example inputs:
- Tests use the crate helper `find_existing_example(day: u32)` which tries these locations in order and picks the first that exists:
  1. `examples/dayNN.txt`
  2. `inputs/dayNNexamples.txt` (the repository's existing naming)
  3. `inputs/dayNN.example.txt`

This makes the test suite flexible: you can keep existing example files in `inputs/` or move them into `examples/`.

Writing tests:
- Because each day's logic lives in `src/days/dayNN.rs` and the day struct implements the `AdventDay` trait, tests can call `DayNN.part1(&input)` and `DayNN.part2(&input)` directly without spawning a binary or running the full process. This yields fast, reliable unit/integration tests.

An example test (already included) asserts Day03's example answers:
```
Part1: 357
Part2: 3121910778619
```

## Project layout and QoL features

- `src/days/dayNN.rs` — library implementation for each day; exposes `pub struct DayNN`.
- `src/bin/dayNN.rs` — tiny binary wrapper that imports `DayNN` and calls `aoc2025::advent_main!(DayNN);`.
- `src/infer_day.rs` — macro that infers day number from the filename for binaries.
- `src/lib.rs` — common helpers:
  - `input_for(day)` / `example_for(day)` — canonical paths
  - `find_existing_example(day)` — search helper used by runner and tests
  - `open(path)` — platform opener for prompts
  - `advent_main!` — macro that implements the standard `main` behavior for day binaries
- `tests/examples.rs` — integration tests that validate example behavior and expected answers.

Optional follow-ups I can do (pick any):
- Move the repository's example files into `examples/` and update references (makes convention consistent).
- Add exact expected assertions for other days (Day01, Day02) if you provide expected outputs.
- Add a `runner` binary that can run any day by number and supports the same flags.

## Credits
Many things to the team behind AOC, visit their [about page here.](https://adventofcode.com/2025/about)
Puzzles, Code, & Design: [Eric Wastl](https://was.tl/)

Beta Testing:

* Ben Lucek
[JP Burke](http://thespaceabove.us/)
* Aneurysm9
* Andrew Skalski
Community Managers: [Danielle Lucek](https://reddit.com/message/compose/?to=/r/adventofcode) and Aneurysm9

Playing: [Me!](https://pixelsaver.github.io/PixelSaver)