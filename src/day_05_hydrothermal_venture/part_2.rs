use std::io::{self, BufRead};

#[derive(Eq, Hash, PartialEq)]
struct Point(u16, u16);

impl std::str::FromStr for Point {
    type Err = &'static str;

    fn from_str(coordinates: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = coordinates
            .split_once(',')
            .ok_or("Invalid format. Expected coordinates in the format: x,y")?;

        let x: u16 = x_str.parse().map_err(|_| "Invalid x-coordinate.")?;
        let y: u16 = y_str.parse().map_err(|_| "Invalid y-coordinate.")?;

        Ok(Self(x, y))
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl std::str::FromStr for Line {
    type Err = &'static str;

    fn from_str(line_coordinates: &str) -> Result<Self, Self::Err> {
        let (start, end) = line_coordinates
            .split_once(" -> ")
            .expect("Should contain two sets of coordinates in format: x1,y1 -> x2,y2");
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn process_data(path: &str) -> Vec<Line> {
    let file = std::fs::File::open(path)
        .expect("The input file should be placed in the input folder beforehand");
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<Line>().unwrap())
        .collect::<Vec<Line>>()
}

fn calculate_number_of_columns_and_rows(vents: &[Line]) -> (usize, usize) {
    let (column, row) = vents.iter().fold((0, 0), |(max_x, max_y), vent| {
        let Point(x1, y1) = vent.start;
        let Point(x2, y2) = vent.end;
        (max_x.max(x1).max(x2), max_y.max(y1).max(y2))
    });
    (column as usize + 1, row as usize + 1)
}

fn calculate_step(value: u16, other_value: u16) -> i32 {
    match value.cmp(&other_value) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

fn count_vent_overlaps(vents: Vec<Line>) -> usize {
    let (number_of_columns, number_of_rows) = calculate_number_of_columns_and_rows(&vents);
    let mut vent_map = vec![0u16; number_of_rows * number_of_columns];

    for vent in vents {
        let Point(x1, y1) = vent.start;
        let Point(x2, y2) = vent.end;
        let dx = calculate_step(x1, x2);
        let dy = calculate_step(y1, y2);
        let mut x = x1 as i32;
        let mut y = y1 as i32;

        while x != x2 as i32 || y != y2 as i32 {
            vent_map[x as usize + y as usize * number_of_columns] += 1;
            x += dx;
            y += dy;
        }
        vent_map[x as usize + y as usize * number_of_columns] += 1;
    }

    vent_map.iter().filter(|&&value| value > 1).count()
}

pub fn solve() -> String {
    let hydrothermal_vents = process_data("./input/05.txt");
    let result = count_vent_overlaps(hydrothermal_vents);
    format!("Day 5: Hydrothermal Venture (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_05_hydrothermal_venture::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let hydrothermal_vents = process_data("./test_input/05.txt");
        assert_eq!(hydrothermal_vents.len(), 10);
        let result = count_vent_overlaps(hydrothermal_vents);
        assert_eq!(result, 12);
    }
}
