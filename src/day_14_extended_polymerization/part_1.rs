use std::collections::HashMap;

fn process_data(path: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut lines = file_content.lines();

    let polymer_template: Vec<char> = lines.next().unwrap().chars().collect();

    let insertion_rules = lines
        .skip(1)
        .map(|line| {
            let (char_pair, insert) = line.split_once(" -> ").unwrap();
            let mut char_pair = char_pair.chars();
            let char_pair = (char_pair.next().unwrap(), char_pair.next().unwrap());
            let insert_char = insert.chars().next().unwrap();
            (char_pair, insert_char)
        })
        .collect();

    (polymer_template, insertion_rules)
}

fn grow_polymer(mut polymer: Vec<char>, rules: &HashMap<(char, char), char>) -> usize {
    for _ in 0..10 {
        let mut next_polymer: Vec<char> = Vec::new();

        polymer.windows(2).for_each(|pair| {
            let first_char = pair.first().unwrap();
            let second_char = pair.last().unwrap();
            next_polymer.push(*first_char);
            next_polymer.push(rules[&(*first_char, *second_char)]);
        });
        next_polymer.push(*polymer.last().unwrap());
        polymer = next_polymer;
    }

    let mut counters = HashMap::new();

    for &char in &polymer {
        *counters.entry(char).or_insert(0) += 1;
    }

    let (min, max) = counters
        .values()
        .fold((usize::MAX, 0), |(min, max), &count| {
            (min.min(count), max.max(count))
        });

    max - min
}

pub fn solve() -> String {
    let (polymer, rules) = process_data("./input/14.txt");
    let result = grow_polymer(polymer, &rules);
    format!(
        "Day 14: Extended Polymerization (Part 1) answer: {}",
        result
    )
}

#[cfg(test)]
mod tests {
    use crate::day_14_extended_polymerization::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (polymer, rules) = process_data("./test_input/14.txt");
        assert_eq!(polymer.len(), 4);
        assert_eq!(rules.len(), 16);
        let result = grow_polymer(polymer, &rules);
        assert_eq!(result, 1588);
    }
}
