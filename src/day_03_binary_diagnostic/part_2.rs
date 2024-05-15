use std::io::{self, BufRead};

enum Mode {
    LeastCommon,
    MostCommon,
}

fn process_data(path: &str) -> Vec<String> {
    let file = std::fs::File::open(path).unwrap();

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}

fn compare_number_of_ones_against_zeroes(report: &[String], index: usize) -> std::cmp::Ordering {
    let count_of_ones: usize = report
        .iter()
        .filter(|binary_number| binary_number.chars().nth(index).unwrap() == '1')
        .count();

    count_of_ones.cmp(&(report.len() - count_of_ones))
}

fn filter_report(mut report: Vec<String>, mode: Mode) -> String {
    let mut index: usize = 0;

    while report.len() > 1 {
        let cmp_result = compare_number_of_ones_against_zeroes(&report, index);

        let digit_to_keep: u16 = match mode {
            Mode::MostCommon => match cmp_result {
                std::cmp::Ordering::Less => 0,
                _ => 1,
            },
            Mode::LeastCommon => match cmp_result {
                std::cmp::Ordering::Less => 1,
                _ => 0,
            },
        };

        report.retain(|binary_number| {
            binary_number.chars().nth(index).unwrap() as u16 - '0' as u16 == digit_to_keep
        });

        index += 1;
    }

    report.remove(0)
}

pub fn solve() -> String {
    let diagnostic_report = process_data("./input/03.txt");
    let oxygen = filter_report(diagnostic_report.clone(), Mode::MostCommon);
    let co2 = filter_report(diagnostic_report, Mode::LeastCommon);
    format!(
        "Day 3: Binary Diagnostic (Part 2) answer: {:?}.",
        u32::from_str_radix(&oxygen, 2).unwrap() * u32::from_str_radix(&co2, 2).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use crate::day_03_binary_diagnostic::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let diagnostic_report = process_data("./test_input/03.txt");
        assert_eq!(diagnostic_report.len(), 12);
        let oxygen = filter_report(diagnostic_report.clone(), Mode::MostCommon);
        let co2 = filter_report(diagnostic_report, Mode::LeastCommon);
        assert_eq!(u32::from_str_radix(&oxygen, 2).unwrap(), 23);
        assert_eq!(u32::from_str_radix(&co2, 2).unwrap(), 10);
    }
}
