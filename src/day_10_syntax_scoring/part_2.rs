use std::collections::HashMap;

fn process_data(path: &str) -> Vec<String> {
    let file_content = std::fs::read_to_string(path);
    file_content.unwrap().lines().map(str::to_owned).collect()
}

fn find_missing_symbols(line: &str) -> Option<Vec<char>> {
    let symbol_pairs: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
            stack.push(symbol_pairs[&c]);
        } else if let Some(last_char) = stack.last() {
            if c == *last_char {
                stack.pop();
            } else {
                return None;
            }
        }
    }

    stack.reverse();
    Some(stack)
}

fn calculate_autocomplete_score(log: &[String]) -> usize {
    let score_map: HashMap<char, usize> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut autocomplete_scores: Vec<usize> = log
        .iter()
        .filter_map(|line| {
            find_missing_symbols(line)
                .map(|result| result.iter().fold(0, |score, &c| score * 5 + score_map[&c]))
        })
        .collect::<Vec<usize>>();

    autocomplete_scores.sort();
    autocomplete_scores[autocomplete_scores.len() / 2]
}

pub fn solve() -> String {
    let log = process_data("./input/10.txt");
    let result = calculate_autocomplete_score(&log);
    format!("Day 10: Syntax Scoring (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_10_syntax_scoring::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let log = process_data("./test_input/10.txt");
        assert_eq!(log.len(), 10);
        let result = calculate_autocomplete_score(&log);
        assert_eq!(result, 288957);
    }
}
