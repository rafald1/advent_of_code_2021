use std::collections::HashMap;

struct Input {
    polymer_original_form: String,
    polymer_pair_form: HashMap<(char, char), usize>,
    insertion_rules: HashMap<(char, char), char>,
}

fn process_data(path: &str) -> Input {
    let file_content = std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand");
    let mut lines = file_content.lines();

    let polymer_original_form: String = lines
        .next()
        .expect("The first line should contain the original polymer form")
        .to_owned();

    let polymer_pair_form: HashMap<(char, char), usize> = polymer_original_form
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(HashMap::new(), |mut acc, pair| {
            let first_char = pair.first().unwrap();
            let second_char = pair.last().unwrap();
            *acc.entry((*first_char, *second_char)).or_insert(0) += 1;
            acc
        });

    let insertion_rules: HashMap<(char, char), char> = lines
        .skip(1)
        .map(|line| {
            let (char_pair, insert) = line.split_once(" -> ")
                .expect("Each line should contain a pair of chars and an insertion char separated by ' -> '");
            let mut char_pair = char_pair.chars();
            let first_char = char_pair.next().unwrap();
            let second_char = char_pair.next().unwrap();
            let insert_char = insert.chars().next().unwrap();
            ((first_char, second_char), insert_char)
        })
        .collect();

    Input {
        polymer_original_form,
        polymer_pair_form,
        insertion_rules,
    }
}

fn grow_polymer(
    polymer_pair_form: &mut HashMap<(char, char), usize>,
    insertion_rules: &HashMap<(char, char), char>,
    number_of_rounds: usize,
) {
    for _ in 0..number_of_rounds {
        let mut next_polymer_pair_form: HashMap<(char, char), usize> = HashMap::new();

        for (&(first_char, second_char), &counter) in polymer_pair_form.iter() {
            let insert_char = insertion_rules
                .get(&(first_char, second_char))
                .expect("Insertion rule should exist for every pair");

            *next_polymer_pair_form
                .entry((first_char, *insert_char))
                .or_insert(0) += counter;

            *next_polymer_pair_form
                .entry((*insert_char, second_char))
                .or_insert(0) += counter;
        }

        *polymer_pair_form = next_polymer_pair_form;
    }
}

fn count_each_char_occurrence(
    polymer_original_form: &str,
    polymer_pair_form: &HashMap<(char, char), usize>,
) -> HashMap<char, usize> {
    let original_polymer_first_char = polymer_original_form.chars().next().unwrap();
    let mut char_counters: HashMap<char, usize> = HashMap::from([(original_polymer_first_char, 1)]);

    for ((_, second_char), &counter) in polymer_pair_form {
        *char_counters.entry(*second_char).or_insert(0) += counter;
    }

    char_counters
}

fn calculate_score_from_min_and_max_value(char_counters: &HashMap<char, usize>) -> usize {
    let (min, max) = char_counters
        .values()
        .fold((usize::MAX, 0), |(min, max), &counter| {
            (min.min(counter), max.max(counter))
        });

    max - min
}

pub fn solve() -> String {
    let mut input = process_data("./input/14.txt");
    grow_polymer(&mut input.polymer_pair_form, &input.insertion_rules, 40);
    let char_counters =
        count_each_char_occurrence(&input.polymer_original_form, &input.polymer_pair_form);
    let result = calculate_score_from_min_and_max_value(&char_counters);
    format!(
        "Day 14: Extended Polymerization (Part 2) answer: {}",
        result
    )
}

#[cfg(test)]
mod tests {
    use crate::day_14_extended_polymerization::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let mut input = process_data("./test_input/14.txt");
        assert_eq!(input.polymer_original_form.len(), 4);
        assert_eq!(input.polymer_pair_form.len(), 3);
        assert_eq!(input.insertion_rules.len(), 16);
        grow_polymer(&mut input.polymer_pair_form, &input.insertion_rules, 40);
        assert_eq!(input.polymer_pair_form.len(), 15);
        let char_counters =
            count_each_char_occurrence(&input.polymer_original_form, &input.polymer_pair_form);
        assert_eq!(char_counters.len(), 4);
        let result = calculate_score_from_min_and_max_value(&char_counters);
        assert_eq!(result, 2188189693529);
    }
}
