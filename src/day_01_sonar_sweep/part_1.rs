use std::io::{self, BufRead};

fn process_data(path: &str) -> Vec<u32> {
    let file = std::fs::File::open(path)
        .expect("The input file should be placed in the input folder beforehand");

    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    let mut sweep_report = Vec::<u32>::new();

    while reader.read_line(&mut line).unwrap() != 0 {
        let number = line
            .trim_end()
            .parse()
            .expect("Current line should contain only digits");
        sweep_report.push(number);
        line.clear();
    }

    sweep_report
}

fn count_number_of_depth_measurement_increases(sweep_report: Vec<u32>) -> usize {
    sweep_report
        .iter()
        .zip(sweep_report.iter().skip(1))
        .filter(|(&x, &y)| x < y)
        .count()
}

pub fn solve() -> String {
    let report = process_data("./input/01.txt");
    let result = count_number_of_depth_measurement_increases(report);
    format!("Day 1: Sonar Sweep (Part 1) answer: {}.", result)
}

#[cfg(test)]
mod tests {
    use crate::day_01_sonar_sweep::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let report = process_data("./test_input/01.txt");
        assert_eq!(report.len(), 10);
        let result = count_number_of_depth_measurement_increases(report);
        assert_eq!(result, 7);
    }
}
