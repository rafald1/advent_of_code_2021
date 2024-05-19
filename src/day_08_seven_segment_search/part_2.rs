fn process_data(path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .map(|line| {
            let (signal_patterns, digit_outputs) = line.split_once(" | ").unwrap();
            (
                signal_patterns
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect(),
                digit_outputs
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect(),
            )
        })
        .collect()
}

fn calculate_output_values_sum(
    sets_of_patterns_and_outputs: &[(Vec<String>, Vec<String>)],
) -> usize {
    let mut sum_of_output_values = 0usize;

    for (patterns, digit_outputs) in sets_of_patterns_and_outputs {
        let pattern_of_one = patterns.iter().find(|&pattern| pattern.len() == 2).unwrap();
        let pattern_of_four = patterns.iter().find(|&pattern| pattern.len() == 4).unwrap();
        let mut decoded_digit_output_values: String = String::new();

        for digit_output in digit_outputs {
            let decoded_digit_output_value = match (
                digit_output.len(),
                digit_output
                    .chars()
                    .filter(|&char| pattern_of_one.contains(char))
                    .count(),
                digit_output
                    .chars()
                    .filter(|&char| pattern_of_four.contains(char))
                    .count(),
            ) {
                (2, _, _) => "1",
                (3, _, _) => "7",
                (4, _, _) => "4",
                (7, _, _) => "8",
                (6, _, 4) => "9",
                (6, 1, _) => "6",
                (6, 2, _) => "0",
                (5, 2, _) => "3",
                (5, _, 2) => "2",
                (5, _, 3) => "5",
                _ => unreachable!(),
            };

            decoded_digit_output_values += decoded_digit_output_value;
        }

        sum_of_output_values += decoded_digit_output_values.parse::<usize>().unwrap();
    }

    sum_of_output_values
}

pub fn solve() -> String {
    let signal_patterns_and_output = process_data("./input/08.txt");
    let result = calculate_output_values_sum(&signal_patterns_and_output);
    format!("Day 8: Seven Segment Search (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_08_seven_segment_search::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let lit_digits = process_data("./test_input/08.txt");
        assert_eq!(lit_digits.len(), 10);
        let result = calculate_output_values_sum(&lit_digits);
        assert_eq!(result, 61229);
    }
}
