use std::collections::HashMap;
use std::io::{self, BufRead};

struct Board {
    numbers: HashMap<u8, (u8, u8)>,
    marked_per_row_count: [u8; 5],
    marked_per_column_count: [u8; 5],
}

impl Board {
    fn from(numbers: &[u8]) -> Self {
        let mut numbers_map: HashMap<u8, (u8, u8)> = HashMap::new();

        for (index, number) in numbers.iter().enumerate() {
            numbers_map.insert(*number, (index as u8 / 5u8, index as u8 % 5u8));
        }

        Board {
            numbers: numbers_map,
            marked_per_column_count: [0; 5],
            marked_per_row_count: [0; 5],
        }
    }

    fn bingo(&mut self, number: u8) -> bool {
        if let Some((column, row)) = self.numbers.remove(&number) {
            self.marked_per_column_count[column as usize] += 1;
            self.marked_per_row_count[row as usize] += 1;

            if self.marked_per_column_count[column as usize] == 5
                || self.marked_per_row_count[row as usize] == 5
            {
                return true;
            }
        }
        false
    }

    fn calculate_unmarked_numbers_sum(&self) -> u32 {
        self.numbers.keys().map(|key| *key as u32).sum::<u32>()
    }
}

fn process_data(path: &str) -> (Vec<u8>, Vec<Board>) {
    let file = std::fs::File::open(path)
        .expect("The input file should be placed in the input folder beforehand");
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    let drawn_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u8>().unwrap())
        .collect();
    let bingo_boards = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
        .chunks(5)
        .map(|chunk| chunk.concat())
        .collect::<Vec<Vec<u8>>>()
        .into_iter()
        .map(|board_numbers| Board::from(&board_numbers))
        .collect();

    (drawn_numbers, bingo_boards)
}

fn play_bingo(drawn_numbers: Vec<u8>, mut bingo_boards: Vec<Board>) -> u32 {
    for drawn_number in drawn_numbers {
        for bingo_board in &mut bingo_boards {
            if bingo_board.bingo(drawn_number) {
                return bingo_board.calculate_unmarked_numbers_sum() * drawn_number as u32;
            }
        }
    }

    panic!("No bingo board contained bingo");
}

pub fn solve() -> String {
    let (drawn_numbers, bingo_boards) = process_data("./input/04.txt");
    let result = play_bingo(drawn_numbers, bingo_boards);
    format!("Day 4: Giant Squid (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_04_giant_squid::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (drawn_numbers, bingo_boards) = process_data("./test_input/04.txt");
        assert_eq!(drawn_numbers.len(), 27);
        assert_eq!(bingo_boards.len(), 3);
        let result = play_bingo(drawn_numbers, bingo_boards);
        assert_eq!(result, 4512);
    }
}
