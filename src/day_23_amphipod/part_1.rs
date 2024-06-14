use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Amphipod;

impl Amphipod {
    const AMBER: usize = 1;
    const BRONZE: usize = 2;
    const COPPER: usize = 3;
    const DESERT: usize = 4;
    const MOVEMENT_COST: [usize; 4] = [1, 10, 100, 1000];
}

struct Burrow;

impl Burrow {
    const HALLWAY: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
    const ROOM_ENTRY: [usize; 4] = [2, 4, 6, 8];
    const ROOMS: [[usize; 2]; 4] = [[11, 12], [13, 14], [15, 16], [17, 18]];
}

fn generate_valid_moves(burrow_state: &[usize; 19]) -> Vec<(usize, usize, usize)> {
    let mut valid_moves: Vec<(usize, usize, usize)> = Vec::new();

    for hallway in Burrow::HALLWAY {
        if burrow_state[hallway] > 0 {
            let home_room = burrow_state[hallway] - 1;
            let room_size = 2;
            let mut path = hallway.min(Burrow::ROOM_ENTRY[home_room])
                ..=hallway.max(Burrow::ROOM_ENTRY[home_room]);
            let path_len = path.end() - path.start() + 1;
            let is_path_unobstructed = path.all(|i| i == hallway || burrow_state[i] == 0);
            let room_states = Burrow::ROOMS[home_room].map(|index| burrow_state[index]);
            let can_amphipod_move = room_states
                .iter()
                .all(|state| *state == 0 || *state == home_room + 1);

            if can_amphipod_move && is_path_unobstructed {
                let first_occupied_index = room_states.iter().position(|state| *state > 0);
                let vacant_index = first_occupied_index.unwrap_or(room_size) - 1;

                valid_moves.push((
                    hallway,
                    Burrow::ROOMS[home_room][0] + vacant_index,
                    path_len + vacant_index,
                ));
            }
        } else {
            for room in 0..=3 {
                let mut path =
                    Burrow::ROOM_ENTRY[room].min(hallway)..=Burrow::ROOM_ENTRY[room].max(hallway);
                let path_len = path.end() - path.start() + 1;
                let is_path_unobstructed = path.all(|i| i == hallway || burrow_state[i] == 0);
                let room_states = Burrow::ROOMS[room].map(|index| burrow_state[index]);
                let first_occupied_index = room_states.iter().position(|state| *state > 0);

                if let Some(occupied_index) = first_occupied_index {
                    if is_path_unobstructed {
                        valid_moves.push((
                            Burrow::ROOMS[room][0] + occupied_index,
                            hallway,
                            path_len + occupied_index,
                        ));
                    }
                }
            }
        }
    }

    valid_moves
}

fn calculate_minimum_energy_cost(burrow_state: [usize; 19]) -> usize {
    let finish_condition: [usize; 19] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4];
    let mut p_queue = BinaryHeap::from([(Reverse(0), burrow_state)]);
    let mut minimum_cost: HashMap<[usize; 19], usize> = HashMap::from([(burrow_state, 0)]);

    while let Some((Reverse(cost), state)) = p_queue.pop() {
        if state == finish_condition {
            return cost;
        }

        let valid_moves = generate_valid_moves(&state);

        for (from, to, distance) in valid_moves {
            let mut next_state = state;
            let amphipod = next_state[from] - 1;
            next_state[to] = amphipod + 1;
            next_state[from] = 0;
            let next_cost = cost + distance * Amphipod::MOVEMENT_COST[amphipod];

            if next_cost < *minimum_cost.get(&next_state).unwrap_or(&usize::MAX) {
                minimum_cost.insert(next_state, next_cost);
                p_queue.push((Reverse(next_cost), next_state));
            }
        }
    }

    unreachable!()
}

fn process_data(path: &str) -> [usize; 19] {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut state: [usize; 19] = [0; 19];
    let room_size = 2;

    for (i, b) in file_content
        .bytes()
        .filter(|b| *b >= b'A' && *b <= b'D')
        .enumerate()
    {
        state[11 + room_size * (i % 4) + i / 4] = match b {
            b'A' => Amphipod::AMBER,
            b'B' => Amphipod::BRONZE,
            b'C' => Amphipod::COPPER,
            b'D' => Amphipod::DESERT,
            _ => unreachable!(),
        };
    }

    state
}

pub fn solve() -> String {
    let burrow_state = process_data("./input/23.txt");
    let result = calculate_minimum_energy_cost(burrow_state);
    format!("Day 23: Amphipod (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_23_amphipod::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let burrow_state = process_data("./test_input/23.txt");
        let result = calculate_minimum_energy_cost(burrow_state);
        assert_eq!(result, 12521);
    }
}
