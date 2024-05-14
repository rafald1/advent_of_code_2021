use std::io::{self, BufRead};

enum Command {
    Forward(u8),
    Up(u8),
    Down(u8),
}

impl std::str::FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, value) = s.split_once(' ').unwrap();
        let value = value.parse::<u8>().unwrap();

        match command {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err("Error: Unrecognized command."),
        }
    }
}

fn process_data(path: &str) -> Vec<Command> {
    let file = std::fs::File::open(path).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<Command>>()
}

fn find_submarine_position(planned_course: Vec<Command>) -> u32 {
    let mut x_pos: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for command in planned_course {
        match command {
            Command::Forward(value) => {
                x_pos += value as u32;
                depth += aim * value as u32;
            }
            Command::Up(value) => aim -= value as u32,
            Command::Down(value) => aim += value as u32,
        }
    }

    x_pos * depth
}

pub fn solve() -> String {
    let commands = process_data("./input/02.txt");
    let result = find_submarine_position(commands);
    format!("Day 2: Dive! (Part 2) answer: {}.", result)
}

#[cfg(test)]
mod tests {
    use crate::day_02_dive::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let commands = process_data("./test_input/02.txt");
        assert_eq!(commands.len(), 6);
        let result = find_submarine_position(commands);
        assert_eq!(result, 900);
    }
}
