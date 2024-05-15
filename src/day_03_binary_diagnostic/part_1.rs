use std::io::{self, BufRead};

fn process_data(path: &str) -> Vec<String> {
    let file = std::fs::File::open(path)
        .expect("The input file should be placed in the input folder beforehand");

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}

fn calculate_gamma_and_epsilon_rates(report: Vec<String>) -> (String, String) {
    let report_len = report.len();
    let mut counts: Vec<usize> = vec![0; report[0].len()];

    for binary_number in report {
        for (index, char) in binary_number.chars().enumerate() {
            if char == '1' {
                counts[index] += 1;
            }
        }
    }

    let gamma_rate: String = counts
        .iter()
        .map(|&count| if count > (report_len / 2) { '1' } else { '0' })
        .collect();

    let epsilon_rate: String = gamma_rate
        .chars()
        .map(|char| if char == '0' { '1' } else { '0' })
        .collect();

    (gamma_rate, epsilon_rate)
}

pub fn solve() -> String {
    let diagnostic_report = process_data("./input/03.txt");
    let (gamma, epsilon) = calculate_gamma_and_epsilon_rates(diagnostic_report);
    format!(
        "Day 3: Binary Diagnostic (Part 1) answer: {}.",
        u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use crate::day_03_binary_diagnostic::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let diagnostic_report = process_data("./test_input/03.txt");
        assert_eq!(diagnostic_report.len(), 12);
        let result = calculate_gamma_and_epsilon_rates(diagnostic_report);
        assert_eq!(result, ("10110".to_owned(), "01001".to_owned()));
    }
}
