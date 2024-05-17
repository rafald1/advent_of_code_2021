fn process_data(path: &str) -> [usize; 9] {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .fold([0usize; 9], |mut grouped_fish, fish_value| {
            grouped_fish[fish_value] += 1;
            grouped_fish
        })
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
    let grouped_lanternfish = process_data("./input/06.txt");
    let result = play_fish_game(grouped_lanternfish, 256);
    format!("Day 6: Lanternfish (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_06_lanternfish::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let grouped_lanternfish = process_data("./test_input/06.txt");
        assert_eq!(grouped_lanternfish, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let result = play_fish_game(grouped_lanternfish, 256);
        assert_eq!(result, 26984457539);
    }
}
