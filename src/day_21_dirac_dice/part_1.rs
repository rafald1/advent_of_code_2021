struct Player {
    pawn_pos: usize,
    score: usize,
}

fn process_data(path: &str) -> Vec<Player> {
    let file_content = std::fs::read_to_string(path).unwrap();
    file_content
        .lines()
        .map(|line| Player {
            pawn_pos: line.split_whitespace().last().unwrap().parse().unwrap(),
            score: 0,
        })
        .collect()
}

fn play_dirac_dice(players: &mut Vec<Player>) -> usize {
    let mut die = (1..=100).cycle().enumerate();
    let mut roll = || die.next().unwrap().1;

    loop {
        for player in &mut *players {
            player.pawn_pos = (player.pawn_pos + roll() + roll() + roll() - 1) % 10 + 1;
            player.score += player.pawn_pos;

            if player.score >= 1000 {
                return players.iter().map(|player| player.score).min().unwrap()
                    * die.next().unwrap().0;
            }
        }
    }
}

pub fn solve() -> String {
    let mut players = process_data("./input/21.txt");
    let result = play_dirac_dice(&mut players);
    format!("Day 21: Dirac Dice (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_21_dirac_dice::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let mut players = process_data("./test_input/21.txt");
        assert_eq!(players.len(), 2);
        let result = play_dirac_dice(&mut players);
        assert_eq!(result, 739785);
    }
}
