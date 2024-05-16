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

### [Day 1](https://adventofcode.com/2021/day/1)
I wrestled with the way Rust handles errors, trying different approaches:
- dealing with Rust's `Result` enum by using match and if statements,
- the `.expect()` method,
- the `.unwrap()` method,
- `?` - this approach was the most enigmatic for me at this point, as I hadn't been exposed to it before.

The first puzzle was straightforward.

For part 2, I refactored the `process_data()` function. I arrived at a more concise version, albeit with a loss of performance.

### [Day 2](https://adventofcode.com/2021/day/2)
This puzzle was straightforward as well.

I used an enum to define possible commands. In part 2, I refactored the way I process the input data in an attempt to make it more concise. I implemented `FromStr` for my enum. I lost some details when handling an error message and made the solution slower in the process. Nevertheless, I learned something new and had an additional chance to practice.

### [Day 3](https://adventofcode.com/2021/day/3)
Finally, a puzzle that gave me an opportunity to explore different approaches and spend time refactoring and learning. Refactoring part 2 significantly reduced duplication and made the code more readable.

Thanks to `cargo clippy`, I learned even more:

```text
writing `&Vec` instead of `&[_]` involves a new object where a slice will do

&Vec<String>
^^^^^^^^^^^^ help: change this to: `&[String]`
```

### Update #1

On this note, a git pre-commit hook works as intended, ensuring that Rust's built-in tools are used to help me with my code. I'm including my pre-commit hook in the `git/hooks/` folder.

I have finished wrestling with Rust over my project structure. I'm including a `mod.rs` file in each puzzle solution folder to reduce the number of lines in `main.rs`. This way, I only need to include:
```rust
mod day_01_sonar_sweep;
```
instead of:
```rust
mod day_01_sonar_sweep {
    pub mod part_1;
    pub mod part_2;
}
```
While I understand that moving all puzzle solution folders into a new folder could make it even more concise, I'm fine with adding a `mod.rs` file.

At this stage, my tests for each puzzle are limited to running a solution on test data.

### [Day 4](https://adventofcode.com/2021/day/4)
Processing the input file was the more challenging part for me. It's crazy how you can endlessly chain all these methods when you try to process and shape your data. Just attempting to use these various methods consumed so much of my time. I decided to use a different approach in part 2 for processing data and used std::fs::read_to_string() instead of std::io::BufReader, which in this case allowed me to simplify my code.
