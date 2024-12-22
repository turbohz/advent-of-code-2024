# Advent of Code 2024

Solutions to the **[Advent of Code 2024](https://adventofcode.com/2024)** challenges, using the Rust programming language.

> This document and the solutions are a work in progress,
> and may often be reviewed and reworked.

## Themes

Instead of atacking the problems with quick and dirty solutions,
the intention is to choose approches that are interesting to me.

Here's a list of ideas and techniques motivating their implementation:

- Parsing using [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar)s
- Chaining iterators, remaining _lazy_ as much as possible
- Testing the features for the current beta of [Rust 2024](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html) edition
- Implementing part 2 of each problem
- Using no crates, other that [peg](https://crates.io/crates/peg) and [Itertools](https://crates.io/crates/itertools)
