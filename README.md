Advent of Code
================

Welcome to my first advent of code attempt!

Main aim here is learning, trying things as a beginner in Rust and
sharpening my R skills. I also hope to try the challenges in different
languages such as Python at some stage.

To check that the code is reproducible and to generate a few summary
statistics for each solution, the README.md file is generated with the
literate programming system [Quarto](https://quarto.org/). This allows
benchmarking of each solution using the
[`hyperfine`](https://github.com/sharkdp/hyperfine) Rust crate and
command line tool, plus information on the number of lines of code
written to solve the problem in each attempt, with the
[`cloc`](https://github.com/hrbrmstr/cloc) tool.

To reproduce the results take a look at the contents of
[README.qmd](https://github.com/Robinlovelace/advent-of-code/blob/main/README.qmd)
which contains all the code needed to reproduce the results by running
code in the subfolders and run the benchmarks.

The whole point of this is learning and sharing so any
comments/suggestions/other on these attempts very welcome. Many thanks
to Dustin Carlino who gave me some help on the Rust code, and to Andrea
Gilardi whose [R solutions](https://github.com/agila5/aoc2022/) I part
copied (and wholy copied to show how much faster their solution was than
mine)!

# Rust

| directory         | Benchmark (seconds) | N. code files | Lines of code | Lines of comments |
|:------------------|--------------------:|--------------:|--------------:|------------------:|
| rust//day01       |               0.048 |             2 |            49 |                10 |
| rust//day02       |               0.053 |             2 |            55 |                23 |
| rust//day02_part2 |               0.048 |             2 |            69 |                37 |

# R

| directory       | Benchmark (seconds) | N. code files | Lines of code | Lines of comments |
|:----------------|--------------------:|--------------:|--------------:|------------------:|
| R//day01        |               1.410 |             1 |            12 |                 2 |
| R//day01-agila5 |               0.181 |             1 |             9 |                 2 |

## Reproduce the results:

``` zsh
cd rust/day01
cargo run
```

       Compiling human_format v1.0.3
       Compiling day01 v0.1.0 (/home/robin/learning/advent-of-code/rust/day01)
        Finished dev [unoptimized + debuginfo] target(s) in 0.67s
         Running `target/debug/day01`
    11.73 M calories carried by
    259 elves
    71124 calories carried by the elf with the most
    That was elf # 113!
    The top 3 elves are:
    Elf #113 with 71124 calories
    Elf #124 with 67422 calories
    Elf #182 with 66093 calories
    In total the top 3 elves carried 204639 calories

``` bash
cd rust/day02
cargo run
```

       Compiling day02 v0.1.0 (/home/robin/learning/advent-of-code/rust/day02)
        Finished dev [unoptimized + debuginfo] target(s) in 0.48s
         Running `target/debug/day02`
    Total points: 10404

``` bash
# cd rust/day03
```
