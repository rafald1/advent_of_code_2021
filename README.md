## Advent of Code 2021 Solutions
This repository contains my solutions for the [Advent of Code 2021](https://adventofcode.com/2021) challenge, implemented in Rust. The purpose of this project is to:
- learn Rust,
- showcase my problem-solving approach,
- practice writing clear and readable code,
- practice basic git workflow.

### Directory Structure
- `src/day_xx_puzzle_name/`: Contains Rust scripts for each day's challenge, where 'xx' is the day number and 'puzzle_name' is the name of the puzzle.
- `input/`: Directory where input files are expected to be placed, formatted as: xx.txt. Input files are not included.
- `test_input/`: Directory where test input files, formatted as xx.txt (or xx_y.txt if necessary), are stored.

### Error Handling
Please note that while the solutions aim to solve the given puzzles accurately, they may not handle unexpected input gracefully. The code assumes input files are formatted as specified in each day's puzzle description.

### Preparations
- **IDE:** RustRover has been chosen. I'm using the 2024.1 Early Access Program Edition.

- **Learning:** I have already started to familiarize myself with Rust. Learning about cargo commands such as `fmt`, `clippy`, and `test` made me realize that I already like Rust, and I'm eager to learn more. I've spent some time playing around and learning about these features, and I'm also trying to create a git pre-commit hook that I can use in this project.

  I'm reading:
  - [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  - [Rust By Practice](https://practice.course.rs)
  - [The Rust Performance Book](https://nnethercote.github.io/perf-book/)

Note: I have been side-tracked by neovim, but as a result, I have knowledge about alternative tools. I have installed and spent time trying to configure if it was necessary:
- `alacritty` - a terminal emulator,
- `tmux` - a terminal multiplexer,
- `fish` - a shell,
- `eza` - a ls replacement,
- `neovim` - I learned about LSP and rust-analyzer, making it a viable alternative for writing Rust code. The configurability is overwhelming, as is learning all the shortcuts, commands, and navigation.
