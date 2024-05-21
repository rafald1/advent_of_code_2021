use std::collections::HashMap;

fn process_data(path: &str) -> Vec<String> {
    let file_content = std::fs::read_to_string(path);
    file_content
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

fn find_syntax_error(line: &str) -> Option<char> {
    let symbol_pairs: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
            stack.push(c);
        } else if let Some(last_char) = stack.last() {
            if c == symbol_pairs[last_char] {
                stack.pop();
            } else {
                return Some(c);
            }
        }
    }

    None
}

fn calculate_syntax_error_score(log: &[String]) -> usize {
    let score_map: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    log.iter()
        .map(|line| find_syntax_error(line).map_or(0, |result| score_map[&result]))
        .sum()
}

pub fn solve() -> String {
    let log = process_data("./input/10.txt");
    let result = calculate_syntax_error_score(&log);
    format!("Day 10: Syntax Scoring (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_10_syntax_scoring::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let log = process_data("./test_input/10.txt");
        assert_eq!(log.len(), 10);
        let result = calculate_syntax_error_score(&log);
        assert_eq!(result, 26397);
    }
}
