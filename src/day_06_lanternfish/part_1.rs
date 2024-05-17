fn process_data(path: &str) -> Vec<u8> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

fn group_fish(lanternfish: &[u8]) -> [usize; 9] {
    let mut grouped_fish: [usize; 9] = [0; 9];

    for fish in lanternfish {
        grouped_fish[*fish as usize] += 1;
    }

    grouped_fish
}

fn play_fish_game(mut grouped_fish: [usize; 9], number_of_days: usize) -> usize {
    for _ in 0..number_of_days {
        let new_fish = grouped_fish[0];

        for i in 1..grouped_fish.len() {
            grouped_fish[i - 1] = grouped_fish[i];
        }

        grouped_fish[8] = new_fish;
        grouped_fish[6] += new_fish;
    }

    grouped_fish.iter().sum()
}

pub fn solve() -> String {
    let lanternfish = process_data("./input/06.txt");
    let grouped_fish = group_fish(&lanternfish);
    let result = play_fish_game(grouped_fish, 80);
    format!("Day 6: Lanternfish (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_06_lanternfish::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let lanternfish = process_data("./test_input/06.txt");
        assert_eq!(lanternfish.len(), 5);
        let grouped_fish = group_fish(&lanternfish);
        assert_eq!(grouped_fish, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let result = play_fish_game(grouped_fish, 80);
        assert_eq!(result, 5934);
    }
}
