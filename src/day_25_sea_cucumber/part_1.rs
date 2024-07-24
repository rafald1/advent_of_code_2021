fn process_data(path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn move_sea_cucumbers_east(seafloor: &mut [Vec<char>], sea_cucumber_moved: &mut bool) {
    let width = seafloor[0].len();
    let mut to_move: Vec<(usize, usize)> = Vec::new();

    for (y, row) in seafloor.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '>' && seafloor[y][(x + 1) % width] == '.' {
                to_move.push((x, y));
            }
        }
    }

    *sea_cucumber_moved |= !to_move.is_empty();

    for (x, y) in to_move {
        seafloor[y][x] = '.';
        seafloor[y][(x + 1) % width] = '>';
    }
}

fn move_sea_cucumbers_south(seafloor: &mut [Vec<char>], sea_cucumber_moved: &mut bool) {
    let height = seafloor.len();
    let mut to_move: Vec<(usize, usize)> = Vec::new();

    for (y, row) in seafloor.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'v' && seafloor[(y + 1) % height][x] == '.' {
                to_move.push((x, y));
            }
        }
    }

    *sea_cucumber_moved |= !to_move.is_empty();

    for (x, y) in to_move {
        seafloor[y][x] = '.';
        seafloor[(y + 1) % height][x] = 'v';
    }
}

fn simulate_sea_cucumbers_movement(seafloor: &mut [Vec<char>]) -> u64 {
    for round_counter in 1.. {
        let mut sea_cucumber_moved = false;

        move_sea_cucumbers_east(seafloor, &mut sea_cucumber_moved);
        move_sea_cucumbers_south(seafloor, &mut sea_cucumber_moved);

        if !sea_cucumber_moved {
            return round_counter;
        }
    }

    unreachable!()
}

pub fn solve() -> String {
    let mut seafloor = process_data("./input/25.txt");
    let result = simulate_sea_cucumbers_movement(&mut seafloor);
    format!("Day 25: Sea Cucumber (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_25_sea_cucumber::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let mut seafloor = process_data("./test_input/25.txt");
        let result = simulate_sea_cucumbers_movement(&mut seafloor);
        assert_eq!(result, 58);
    }
}
