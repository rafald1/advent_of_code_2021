use std::io::{self, BufRead};

enum Command {
    Forward(u8),
    Up(u8),
    Down(u8),
}

fn process_data(path: &str) -> Vec<Command> {
    let file = std::fs::File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    let mut planned_course = Vec::<Command>::new();

    while reader.read_line(&mut line).unwrap() != 0 {
        let (command, value) = line.trim_end().split_once(' ').unwrap();
        let value = value.parse::<u8>().unwrap();

        let command_and_value = match command {
            "forward" => Command::Forward(value),
            "up" => Command::Up(value),
            "down" => Command::Down(value),
            _ => panic!("Error: Unrecognized command: {command}."),
        };

        planned_course.push(command_and_value);
        line.clear();
    }

    planned_course
}

fn find_submarine_position(planned_course: Vec<Command>) -> u32 {
    let mut x_pos: u32 = 0;
    let mut depth: u32 = 0;

    for command in planned_course {
        match command {
            Command::Forward(value) => x_pos += value as u32,
            Command::Up(value) => depth -= value as u32,
            Command::Down(value) => depth += value as u32,
        }
    }

    x_pos * depth
}

pub fn solve() -> String {
    let commands = process_data("./input/02.txt");
    let result = find_submarine_position(commands);
    format!("Day 2: Dive! (Part 1) answer: {}.", result)
}

#[cfg(test)]
mod tests {
    use crate::day_02_dive::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let commands = process_data("./test_input/02.txt");
        assert_eq!(commands.len(), 6);
        let result = find_submarine_position(commands);
        assert_eq!(result, 150);
    }
}
