# Description

My solutions to AoC 2023 while learning Rust.

Using https://github.com/gobanos/cargo-aoc for retrieving puzzle inputs and running/benchmarking solutions.

# Usage

## 1. Provide puzzle inputs

Given https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs I've decided to not include puzzle inputs in the repository any more.

You can either
* add the puzzle inputs manually in `input/2023/` (i.e. `day1.txt`), or
* set your AoC session token with `cargo aoc credentials -s <token>` and retrieve your inputs with `./fetch_inputs.sh`

## 2. Running solutions

All solutions:
```
cargo run
```

Last solution (add `-d <day> -y <year>` for a specific one):
```
cargo aoc
```

## Other commands

Run solutions with sample inputs (and other unittests):
```
cargo test
```

Benchmark all solutions (add `-d <day> -y <year>` for a specific one, add `-o` for `gnuplot` output):
```
cargo aoc bench
```
