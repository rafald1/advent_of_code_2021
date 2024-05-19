fn process_data(path: &str) -> Vec<Vec<String>> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .map(|line| line.split_once(" | ").map(|(_, lit_digits)| lit_digits))
        .map(|lit_digits| {
            lit_digits
                .unwrap()
                .split_whitespace()
                .map(|digit| digit.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn count_occurrence_of_certain_digits(digits: &[Vec<String>]) -> usize {
    digits
        .iter()
        .flatten()
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

pub fn solve() -> String {
    let lit_digits = process_data("./input/08.txt");
    let result = count_occurrence_of_certain_digits(&lit_digits);
    format!("Day 8: Seven Segment Search (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_08_seven_segment_search::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let lit_digits = process_data("./test_input/08.txt");
        assert_eq!(lit_digits.len(), 10);
        let result = count_occurrence_of_certain_digits(&lit_digits);
        assert_eq!(result, 26);
    }
}
