fn process_data(path: &str) -> [[u8; 10]; 10] {
    std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Each row should have exactly 10 elements")
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("There should be exactly 10 rows")
}

fn validate_neighbour(nx: i32, ny: i32) -> Option<(usize, usize)> {
    if (0..10).contains(&nx) && (0..10).contains(&ny) {
        Some((nx as usize, ny as usize))
    } else {
        None
    }
}

fn find_neighbours(x: i32, y: i32) -> Vec<(usize, usize)> {
    [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ]
    .iter()
    .filter_map(|&(dx, dy)| validate_neighbour(x + dx, y + dy))
    .collect()
}

fn build_map_of_neighbours_of_each_octopus() -> [[Vec<(usize, usize)>; 10]; 10] {
    std::array::from_fn(|y| std::array::from_fn(|x| find_neighbours(x as i32, y as i32)))
}

fn increment(
    x: usize,
    y: usize,
    octopuses: &mut [[u8; 10]; 10],
    neighbours: &[[Vec<(usize, usize)>; 10]; 10],
    flashed: &mut [[bool; 10]; 10],
) -> usize {
    if flashed[y][x] {
        return 0;
    }

    octopuses[y][x] += 1;

    if octopuses[y][x] > 9 {
        octopuses[y][x] = 0;
        flashed[y][x] = true;
        let mut flash_counter: usize = 1;

        for &(nx, ny) in &neighbours[y][x] {
            flash_counter += increment(nx, ny, octopuses, neighbours, flashed);
        }

        return flash_counter;
    }

    0
}

fn play_game(octopuses: &mut [[u8; 10]; 10], number_of_rounds: usize) -> usize {
    let neighbours_of_each_octopus = build_map_of_neighbours_of_each_octopus();
    let mut flash_counter: usize = 0;

    for _ in 0..number_of_rounds {
        let mut flashed: [[bool; 10]; 10] = [[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                flash_counter +=
                    increment(x, y, octopuses, &neighbours_of_each_octopus, &mut flashed);
            }
        }
    }

    flash_counter
}

pub fn solve() -> String {
    let mut energy_level_of_octopuses = process_data("./input/11.txt");
    let result = play_game(&mut energy_level_of_octopuses, 100);
    format!("Day 11: Dumbo Octopus (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_11_dumbo_octopus::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let mut energy_level_of_octopuses = process_data("./test_input/11.txt");
        let result = play_game(&mut energy_level_of_octopuses, 100);
        assert_eq!(result, 1656);
    }
}
