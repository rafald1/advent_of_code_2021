use std::collections::HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Player {
    pawn_pos: u8,
    score: u8,
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

fn play_quantum_dirac_dice(
    memo: &mut HashMap<(Player, Player), [usize; 2]>,
    active: Player,
    other: Player,
) -> [usize; 2] {
    if other.score >= 21 {
        return [0, 1];
    }

    if let Some(saved_game_score) = memo.get(&(active, other)) {
        return *saved_game_score;
    };

    let mut game_score = [0, 0];

    for [roll_sum, occurrence] in [[3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1]] {
        let pawn_pos = (active.pawn_pos + roll_sum - 1) % 10 + 1;
        let active_player_after_roll = Player {
            pawn_pos,
            score: active.score + pawn_pos,
        };

        let [other_player_local_score, active_player_local_score] =
            play_quantum_dirac_dice(memo, other, active_player_after_roll);

        game_score = [
            game_score[0] + active_player_local_score * occurrence as usize,
            game_score[1] + other_player_local_score * occurrence as usize,
        ];
    }

    memo.insert((active, other), game_score);
    game_score
}

pub fn solve() -> String {
    let players = process_data("./input/21.txt");
    let mut memo: HashMap<(Player, Player), [usize; 2]> = HashMap::new();
    let scores = play_quantum_dirac_dice(&mut memo, players[0], players[1]);
    format!(
        "Day 21: Dirac Dice (Part 2) answer: {}",
        scores.iter().max().unwrap()
    )
}

#[cfg(test)]
mod tests {
    use crate::day_21_dirac_dice::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let players = process_data("./test_input/21.txt");
        assert_eq!(players.len(), 2);
        let mut memo: HashMap<(Player, Player), [usize; 2]> = HashMap::new();
        let scores = play_quantum_dirac_dice(&mut memo, players[0], players[1]);
        assert_eq!(*scores.iter().max().unwrap(), 444356092776315);
    }
}
