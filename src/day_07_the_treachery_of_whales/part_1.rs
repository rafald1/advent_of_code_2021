fn process_data(path: &str) -> Vec<u16> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

fn calculate_fuel_consumption(mut crab_positions: Vec<u16>) -> usize {
    crab_positions.sort();
    let median_pos = crab_positions[crab_positions.len() / 2];
    crab_positions
        .iter()
        .map(|crab_pos| crab_pos.abs_diff(median_pos) as usize)
        .sum()
}

pub fn solve() -> String {
    let crab_positions: Vec<u16> = process_data("./input/07.txt");
    let result = calculate_fuel_consumption(crab_positions);
    format!("Day 7: The Treachery of Whales (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_07_the_treachery_of_whales::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let crab_positions: Vec<u16> = process_data("./test_input/07.txt");
        assert_eq!(crab_positions.len(), 10);
        let result = calculate_fuel_consumption(crab_positions);
        assert_eq!(result, 37)
    }
}
