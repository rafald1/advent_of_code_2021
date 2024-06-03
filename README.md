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

### [Day 5](https://adventofcode.com/2021/day/5)
My first implementation used HashMap, but it was running slower than expected. I also tried HashSet, but ended up using Vec to store vent positions. I decided to use structs and std::str::FromStr to handle input processing. Once again, processing the input file was the more challenging part for me. The chosen approach allowed me to practice error handling.

### [Day 6](https://adventofcode.com/2021/day/6)
This one was straightforward. Part 2 basically didn't require any modification. Part 1 contains my original solution and part 2 is refactored version.

### [Day 7](https://adventofcode.com/2021/day/7)
For part 1, it was easy to notice that the median would produce the correct answer. For part 2, I initially implemented a naive approach that required calculating fuel consumption for reaching each new position and choosing the minimum value. In the second implementation, I improved the calculation time by ceasing the calculation of fuel consumption values when they exceeded the current minimum value. The final version uses the mean to narrow the search for the minimum fuel consumption to two values: one produced by using the mean, and another by using the mean plus one as a new position.

### [Day 8](https://adventofcode.com/2021/day/8)
Part 1 was straightforward, but properly processing the input posed a challenge for me. In part 2, decoding digits in the seven-segment display format was required. Initially, we were able to identify 4 out of 10 digits based on the length of the encoded digit. Then, using the identified digits 1 and 4, we assigned proper values to the remaining formatted digits.

The pattern used to identify all digits is as follows:

- if the formatted digit length is 2, the digit is 1,
- if the formatted digit length is 3, the digit is 7,
- if the formatted digit length is 4, the digit is 4,
- if the formatted digit length is 7, the digit is 8,
- if the formatted digit length is 6 and has 4 common sections with 4, the digit is 9,
- if the formatted digit length is 6 and has 1 common section with 1, the digit is 6,
- if the formatted digit length is 6 and has 2 common sections with 1, the digit is 0,
- if the formatted digit length is 5 and has 2 common sections with 1, the digit is 3,
- if the formatted digit length is 5 and has 2 common sections with 4, the digit is 2,
- if the formatted digit length is 5 and has 3 common sections with 4, the digit is 5.

### [Day 9](https://adventofcode.com/2021/day/9)
Part 1 was straightforward and required checking up to four neighbors of each cell to determine if the required condition was met. Part 2, on the other hand, provided an opportunity to approach the problem in different ways. My first version used a HashMap, the second version used a HashSet, and the third and final version used a Vec to identify basins and their sizes. Although I ended up using Vec, I spent a lot of time experimenting with HashMap and HashSet.

### [Day 10](https://adventofcode.com/2021/day/10)
This was a straightforward and enjoyable puzzle. I used HashMaps to store symbol pairs and score values. It seems like a good time to take a break from solving puzzles and spend some time learning new tricks.

### Update #2
I added two GitHub workflows: one for fmt and Clippy, and another for testing on different operating systems. One of the tests failed on Windows OS due to the problematic code `.split_once("\n\n")`, which I have since fixed. I also added branch protection rules for the main branch. Additionally, I got sidetracked and spent some time learning about property testing and the Proptest framework.

### [Day 11](https://adventofcode.com/2021/day/11)
This was an enjoyable puzzle. I decided to use a 2D array to store the input, which was a 10x10 grid. After some effort and trying different approaches, I found the `.try_into().expect()` combo to transform a vector into an array. The puzzle was more complicated than anticipated, requiring tracking of octopuses that flashed each round. Initially, I used HashSet for tracking, but eventually switched to another 2D array for significant performance gains.

Part 2 required only a few modifications. I optimized the way I stored neighbors of each cell by switching from `[[Vec<(usize, usize)>; 10]; 10]` to `[[[Option<(usize, usize)>; 8]; 10]; 10]`. This sped things up and helped me get more familiar with the `Option` type. This change led Clippy to issue a warning: "very complex type used. Consider factoring parts into type definitions". I followed this advice and used `type Grid<T> = [[T; 10]; 10]`, which indeed simplified things.

### [Day 12](https://adventofcode.com/2021/day/12)
There were a lot of challenges. The first one was processing the data. I still find it confusing when working with `&str` and `String`, and I'm not able to avoid all possible mistakes. Reading `4. Understanding Ownership` from *The Rust Programming Language* made the process a little easier than before. The task here was to find all possible combinations that met certain criteria. My recursive function originally operated fully on `String`, which required a few additional conversions from `&str` to `String`. Switching to `&str` required adding explicit lifetimes (`'a`) in several places, providing an opportunity to practice this unusual concept. Moreover, this change allowed me to speed things up a little bit.

In Part 2, things got interesting. A simple criterion change increased the possible combinations in my case from roughly 4,000 to over 100,000, significantly increasing the time required to find the solution by a factor of 30. Getting a proper solution wasn't straightforward and required a lot of time spent debugging the issue of missing paths for the example input. After finding a working solution, I spent time increasing the readability of the code and reducing the time needed to find the solution. I got rid of the `HashMap` and found a better way to store input data by using a struct and enum to represent each cave and a vector to store them.

### [Day 13](https://adventofcode.com/2021/day/13)
Part 1 was straightforward. I used a struct, an enum, and a HashSet to store input information. Refactoring helped me find a more concise way of folding, which improved readability.

Part 2 required outputting the data to the screen after folding to get the puzzle answer. While refactoring, I replaced the HashSet with a vector and used `.dedup()` to remove duplicates, which improved performance.

### [Day 14](https://adventofcode.com/2021/day/14)
Finishing part 1 and getting access to part 2 made me realize that my brute force solution for part 1 was still too slow, even after replacing HashMaps with arrays, which actually made the solution run four times faster. After thinking about how to solve it differently, I realized that I could insert in bulk only for each different character pair. I changed the representation of the polymer from a `String` to a `HashMap<(char, char), usize>` and stored character pairs with the number of occurrences. I encountered an issue in the final step when counting the occurrence of each individual character. Depending on the method of counting, my final count was missing either the first character or the last character of the original polymer. I thought I could get away without passing additional information when processing the input file, but in cases where the first and last characters were the same, I wasn't able to get a proper count of each character. After a lot of refactoring, the final version emerged. I added the original polymer to be returned as well and used it to calculate the correct character occurrences. I restructured the code to make it more readable.

### [Day 15](https://adventofcode.com/2021/day/15)
This was a shortest path problem in an undirected weighted graph. I wanted to implement Dijkstra's algorithm using a priority queue. I learned a few new things:
* `BinaryHeap` from collections - which is a priority queue implemented with a binary heap. The `.pop()` method from `BinaryHeap` removes the largest item, not the smallest one as in Python.
* `Reverse` from `cmp` - allowed me to get the shortest path instead of the longest path with the way `BinaryHeap` works.
* `wrapping_sub` - was a nice alternative when validating neighbors of the current vertex.

Part 2 increased the number of vertices from 1,000 to 25,000 by duplicating the original ones and introduced a change to the cost of moving between new vertices, which turned out to be challenging to implement concisely.

### [Day 16](https://adventofcode.com/2021/day/16)
This challenge involved writing a parser, parsing the input, and following a set of calculations to get a result from these operations. Part 1 was about correctly splitting the input data (packet) into sub-packets, omitting some details for simplicity's sake. It took a lot of time to go through all the instructions to ensure that the parser was working properly. I used a recursive function to split and parse sub-packets. I decided to use a `String` and consume it while parsing, which allowed me to practice and better understand Rust's unique ownership feature. Getting part 2 working was smooth sailing. I refactored and structured the code to make it easier to understand.
