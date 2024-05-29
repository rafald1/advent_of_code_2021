type Grid<T> = [[T; 10]; 10];

fn process_data(path: &str) -> Grid<u8> {
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

fn find_neighbours(x: i32, y: i32) -> [Option<(usize, usize)>; 8] {
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
    .map(|&(dx, dy)| validate_neighbour(x + dx, y + dy))
    .collect::<Vec<_>>()
    .try_into()
    .expect("There should be 8 neighbours")
}

fn build_map_of_neighbours_of_each_octopus() -> Grid<[Option<(usize, usize)>; 8]> {
    std::array::from_fn(|y| std::array::from_fn(|x| find_neighbours(x as i32, y as i32)))
}

fn increment(
    x: usize,
    y: usize,
    octopuses: &mut Grid<u8>,
    neighbours: &Grid<[Option<(usize, usize)>; 8]>,
    flashed: &mut Grid<bool>,
) {
    if flashed[y][x] {
        return;
    }

    octopuses[y][x] += 1;

    if octopuses[y][x] > 9 {
        octopuses[y][x] = 0;
        flashed[y][x] = true;

        for (nx, ny) in neighbours[y][x].iter().filter_map(|&opt| opt) {
            increment(nx, ny, octopuses, neighbours, flashed);
        }
    }
}

fn play_game(octopuses: &mut Grid<u8>) -> usize {
    let neighbours_of_each_octopus = build_map_of_neighbours_of_each_octopus();
    let mut round: usize = 0;

    loop {
        round += 1;
        let mut flashed: Grid<bool> = [[false; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                increment(x, y, octopuses, &neighbours_of_each_octopus, &mut flashed);
            }
        }

        if flashed.iter().flatten().all(|&value| value) {
            break round;
        }
    }
}

pub fn solve() -> String {
    let mut energy_level_of_octopuses = process_data("./input/11.txt");
    let result = play_game(&mut energy_level_of_octopuses);
    format!("Day 11: Dumbo Octopus (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_11_dumbo_octopus::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let mut energy_level_of_octopuses = process_data("./test_input/11.txt");
        let result = play_game(&mut energy_level_of_octopuses);
        assert_eq!(result, 195);
    }
}
