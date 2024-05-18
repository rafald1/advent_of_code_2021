fn process_data(path: &str) -> Vec<u16> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

fn calculate_fuel_consumption(crab_positions: &[u16]) -> usize {
    let mean_position = crab_positions
        .iter()
        .map(|crab_position| *crab_position as usize)
        .sum::<usize>()
        / crab_positions.len();

    (mean_position..=mean_position + 1)
        .map(|new_pos| {
            crab_positions
                .iter()
                .map(|&crab_pos| {
                    let diff: usize = (crab_pos as usize).abs_diff(new_pos);
                    diff * (1 + diff) / 2
                })
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

pub fn solve() -> String {
    let crab_positions: Vec<u16> = process_data("./input/07.txt");
    let result = calculate_fuel_consumption(&crab_positions);
    format!("Day 7: The Treachery of Whales (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_07_the_treachery_of_whales::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let crab_positions: Vec<u16> = process_data("./test_input/07.txt");
        assert_eq!(crab_positions.len(), 10);
        let result = calculate_fuel_consumption(&crab_positions);
        assert_eq!(result, 168)
    }
}
