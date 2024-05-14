use std::io::{self, BufRead};

fn process_data(path: &str) -> Vec<u32> {
    let file = std::fs::File::open(path).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u32>>()
}

fn count_number_of_measurement_sums_increases(sweep_report: Vec<u32>) -> usize {
    sweep_report
        .windows(3)
        .map(|window| window.iter().sum::<u32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|&pair| pair[0] < pair[1])
        .count()
}

pub fn solve() -> String {
    let report = process_data("./input/01.txt");
    let result = count_number_of_measurement_sums_increases(report);
    format!("Day 1: Sonar Sweep (Part 2) answer: {}.", result)
}

#[cfg(test)]
mod tests {
    use crate::day_01_sonar_sweep::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let report = process_data("./test_input/01.txt");
        assert_eq!(report.len(), 10);
        let result = count_number_of_measurement_sums_increases(report);
        assert_eq!(result, 5);
    }
}
