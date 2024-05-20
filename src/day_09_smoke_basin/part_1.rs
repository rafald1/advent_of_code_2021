fn process_data(path: &str) -> Vec<Vec<u8>> {
    let file_content = std::fs::read_to_string(path);
    let mut heightmap: Vec<Vec<u8>> = Vec::new();

    for line in file_content.unwrap().lines() {
        let mut row: Vec<u8> = Vec::new();

        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as u8);
        }

        heightmap.push(row);
    }

    heightmap
}

fn is_in_bounds(heightmap: &[Vec<u8>], nx: i32, ny: i32) -> bool {
    nx >= 0 && ny >= 0 && nx < heightmap[0].len() as i32 && ny < heightmap.len() as i32
}

fn calculate_sum_of_risk_levels_of_low_points(heightmap: &[Vec<u8>]) -> usize {
    let mut sum_of_risk_levels_of_low_points = 0usize;

    for (y, row) in heightmap.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let is_not_low = [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().any(|&(dx, dy)| {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                is_in_bounds(heightmap, nx, ny) && height >= &heightmap[ny as usize][nx as usize]
            });

            if !is_not_low {
                sum_of_risk_levels_of_low_points += *height as usize + 1
            }
        }
    }

    sum_of_risk_levels_of_low_points
}

pub fn solve() -> String {
    let heightmap = process_data("./input/09.txt");
    let result = calculate_sum_of_risk_levels_of_low_points(&heightmap);
    format!("Day 9: Smoke Basin (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_09_smoke_basin::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let heightmap = process_data("./test_input/09.txt");
        assert_eq!(heightmap.len(), 5);
        let result = calculate_sum_of_risk_levels_of_low_points(&heightmap);
        assert_eq!(result, 15)
    }
}
