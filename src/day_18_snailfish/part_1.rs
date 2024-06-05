fn process_data(path: &str) -> Vec<Vec<(u8, usize)>> {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut snailfish_numbers: Vec<Vec<(u8, usize)>> = Vec::new();

    for line in file_content.lines() {
        let mut depth = 0;
        let mut snailfish_number = Vec::new();
        for c in line.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                digit if digit.is_ascii_digit() => {
                    let value = c.to_digit(10).unwrap() as usize;
                    snailfish_number.push((depth, value));
                }
                _ => {}
            }
        }
        snailfish_numbers.push(snailfish_number);
    }
    snailfish_numbers
}

fn join_two_snailfish_numbers(lhs: &mut Vec<(u8, usize)>, rhs: Vec<(u8, usize)>) {
    lhs.extend(rhs);
    lhs.iter_mut().for_each(|(depth, _)| *depth += 1);
}

fn explode_snailfish_number(snailfish_number: &mut Vec<(u8, usize)>) -> bool {
    for index in 0..snailfish_number.len() - 1 {
        let (left, right) = snailfish_number.split_at_mut(index + 1);
        let (lhs_depth, lhs_value) = left[index];
        let (rhs_depth, rhs_value) = right[0];

        if lhs_depth == 5 && rhs_depth == 5 {
            if index > 0 {
                left[index - 1].1 += lhs_value;
            }

            if right.get(1).is_some() {
                right[1].1 += rhs_value;
            }

            snailfish_number.splice(index..=index + 1, [(4, 0)]);
            return true;
        }
    }
    false
}

fn split_snailfish_number(snailfish_number: &mut Vec<(u8, usize)>) -> bool {
    for index in 0..snailfish_number.len() {
        if snailfish_number[index].1 > 9 {
            let (depth, value) = snailfish_number[index];

            snailfish_number.splice(
                index..=index,
                [(depth + 1, value / 2), (depth + 1, value - value / 2)],
            );
            return true;
        }
    }
    false
}

fn calculate_magnitude(snailfish_number: &mut Vec<(u8, usize)>) {
    while snailfish_number.len() > 1 {
        let mut index = 0;

        while index < snailfish_number.len() - 1 {
            let (left, right) = snailfish_number.split_at_mut(index + 1);
            let (lhs_depth, lhs_value) = &left[index];
            let (rhs_depth, rhs_value) = &right[0];

            if lhs_depth == rhs_depth {
                let new_depth = lhs_depth - 1;
                let new_value = 3 * lhs_value + 2 * rhs_value;
                snailfish_number.splice(index..=index + 1, [(new_depth, new_value)]);
                break;
            }

            index += 1;
        }
    }
}

fn sum_up_snailfish_numbers(snailfish_numbers: &mut Vec<Vec<(u8, usize)>>) -> usize {
    let mut lhs_sfn = snailfish_numbers.remove(0);

    while !snailfish_numbers.is_empty() {
        let rhs_sfn = snailfish_numbers.remove(0);
        join_two_snailfish_numbers(&mut lhs_sfn, rhs_sfn);
        while explode_snailfish_number(&mut lhs_sfn) || split_snailfish_number(&mut lhs_sfn) {}
    }

    calculate_magnitude(&mut lhs_sfn);
    lhs_sfn[0].1
}

pub fn solve() -> String {
    let mut snailfish_numbers = process_data("./input/18.txt");
    let result = sum_up_snailfish_numbers(&mut snailfish_numbers);
    format!("Day 18: Snailfish (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_18_snailfish::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let mut snailfish_numbers = process_data("./test_input/18.txt");
        assert_eq!(snailfish_numbers.len(), 10);
        assert_eq!(
            snailfish_numbers[7],
            Vec::from([(2, 9), (2, 3), (3, 9), (3, 9), (3, 6), (4, 4), (4, 9)])
        );
        let result = sum_up_snailfish_numbers(&mut snailfish_numbers);
        assert_eq!(result, 4140);
    }

    #[test]
    fn join_snailfish_numbers_together() {
        let mut sfn_1: Vec<(u8, usize)> = Vec::from([(1, 1), (1, 2)]);
        let sfn_2: Vec<(u8, usize)> = Vec::from([(2, 3), (2, 4), (1, 5)]);
        let result: Vec<(u8, usize)> = Vec::from([(2, 1), (2, 2), (3, 3), (3, 4), (2, 5)]);

        join_two_snailfish_numbers(&mut sfn_1, sfn_2);
        assert_eq!(*sfn_1, result);
    }

    #[test]
    fn check_if_explode_produces_correct_form() {
        let mut sfn: Vec<(u8, usize)> = Vec::from([(5, 9), (5, 8), (4, 1), (3, 2), (2, 3), (1, 4)]);
        let result: Vec<(u8, usize)> = Vec::from([(4, 0), (4, 9), (3, 2), (2, 3), (1, 4)]);
        explode_snailfish_number(&mut sfn);
        assert_eq!(*sfn, result);
    }

    #[test]
    fn check_if_split_produces_correct_form() {
        let mut sfn: Vec<(u8, usize)> = Vec::from([(3, 15), (4, 0), (4, 13)]);
        let mid_form: Vec<(u8, usize)> = Vec::from([(4, 7), (4, 8), (4, 0), (4, 13)]);
        split_snailfish_number(&mut sfn);
        assert_eq!(*sfn, mid_form);

        let final_form: Vec<(u8, usize)> = Vec::from([(4, 7), (4, 8), (4, 0), (5, 6), (5, 7)]);
        split_snailfish_number(&mut sfn);
        assert_eq!(*sfn, final_form);
    }

    #[test]
    fn check_if_magnitude_is_calculated_correctly() {
        let mut sfn: Vec<(u8, usize)> = Vec::from([(1, 9), (1, 1)]);
        calculate_magnitude(&mut sfn);
        assert_eq!(sfn[0].1, 29);

        sfn = Vec::from([(2, 9), (2, 1), (2, 1), (2, 9)]);
        calculate_magnitude(&mut sfn);
        assert_eq!(sfn[0].1, 129);
    }
}
