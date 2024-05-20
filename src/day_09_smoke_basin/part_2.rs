fn process_data(path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(path);

    file_content
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_in_bounds(heightmap: &[Vec<char>], nx: isize, ny: isize) -> bool {
    nx >= 0 && ny >= 0 && nx < heightmap[0].len() as isize && ny < heightmap.len() as isize
}

fn calculate_basin_size(heightmap: &mut [Vec<char>], start_x: usize, start_y: usize) -> usize {
    let mut local_basin: Vec<(usize, usize)> = Vec::new();
    local_basin.push((start_x, start_y));
    heightmap[start_y][start_x] = 'x';
    let mut basin_size: usize = 1;

    while let Some((x, y)) = local_basin.pop() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if is_in_bounds(heightmap, nx, ny)
                && heightmap[ny as usize][nx as usize] != '9'
                && heightmap[ny as usize][nx as usize] != 'x'
            {
                local_basin.push((nx as usize, ny as usize));
                heightmap[ny as usize][nx as usize] = 'x';
                basin_size += 1;
            }
        }
    }

    basin_size
}

fn calculate_sizes_of_three_largest_basins(heightmap: &mut [Vec<char>]) -> usize {
    let mut basin_sizes = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            if heightmap[y][x] != '9' && heightmap[y][x] != 'x' {
                let basin_size = calculate_basin_size(heightmap, x, y);
                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product::<usize>()
}

pub fn solve() -> String {
    let mut heightmap = process_data("./input/09.txt");
    let result = calculate_sizes_of_three_largest_basins(&mut heightmap);
    format!("Day 9: Smoke Basin (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_09_smoke_basin::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let mut heightmap = process_data("./test_input/09.txt");
        assert_eq!(heightmap.len(), 5);
        let result = calculate_sizes_of_three_largest_basins(&mut heightmap);
        assert_eq!(result, 1134)
    }
}
