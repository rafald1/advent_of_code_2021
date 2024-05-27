use std::collections::HashMap;

struct Board {
    numbers: HashMap<u8, (u8, u8)>,
    marked_per_row_count: [u8; 5],
    marked_per_column_count: [u8; 5],
    consumed: bool,
}

impl Board {
    fn from(numbers: &[Vec<u8>]) -> Board {
        let mut numbers_map: HashMap<u8, (u8, u8)> = HashMap::new();

        for (index, number) in numbers.iter().flatten().enumerate() {
            numbers_map.insert(*number, (index as u8 / 5u8, index as u8 % 5u8));
        }

        Board {
            numbers: numbers_map,
            marked_per_column_count: [0; 5],
            marked_per_row_count: [0; 5],
            consumed: false,
        }
    }

    fn bingo(&mut self, number: u8) -> bool {
        if let Some((column, row)) = self.numbers.remove(&number) {
            self.marked_per_column_count[column as usize] += 1;
            self.marked_per_row_count[row as usize] += 1;
            if self.marked_per_column_count[column as usize] == 5
                || self.marked_per_row_count[row as usize] == 5
            {
                self.consumed = true;
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
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut lines = file_content.lines();

    let drawn_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u8>().unwrap())
        .collect();
    let bingo_boards = lines
        .filter(|&line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
        .chunks(5)
        .map(Board::from)
        .collect::<Vec<Board>>();

    (drawn_numbers, bingo_boards)
}

fn play_bingo(drawn_numbers: Vec<u8>, mut bingo_boards: Vec<Board>) -> u32 {
    let mut win_record: Vec<u32> = Vec::new();

    for drawn_number in drawn_numbers {
        for bingo_board in &mut bingo_boards {
            if bingo_board.bingo(drawn_number) {
                let score = bingo_board.calculate_unmarked_numbers_sum() * drawn_number as u32;
                win_record.push(score);
            }
        }
        bingo_boards.retain(|bingo_board| !bingo_board.consumed);
    }

    *win_record.last().unwrap()
}

pub fn solve() -> String {
    let (drawn_numbers, bingo_boards) = process_data("./input/04.txt");
    let result = play_bingo(drawn_numbers, bingo_boards);
    format!("Day 4: Giant Squid (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_04_giant_squid::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let (drawn_numbers, bingo_boards) = process_data("./test_input/04.txt");
        assert_eq!(drawn_numbers.len(), 27);
        assert_eq!(bingo_boards.len(), 3);
        let result = play_bingo(drawn_numbers, bingo_boards);
        assert_eq!(result, 1924);
    }
}
