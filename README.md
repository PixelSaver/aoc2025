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