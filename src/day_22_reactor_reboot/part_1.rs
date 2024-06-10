use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Instruction {
    is_on: bool,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
}

fn process_data(path: &str) -> Vec<Instruction> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .map(|line| {
            let (is_on, rest) = line.split_once(' ').unwrap();
            let ranges: Vec<i32> = rest
                .split(',')
                .flat_map(|range| range[2..].split("..").map(|num| num.parse().unwrap()))
                .collect();
            let [x1, x2, y1, y2, z1, z2]: [i32; 6] = ranges.try_into().unwrap();
            Instruction {
                is_on: is_on == "on",
                x_range: x1..=x2,
                y_range: y1..=y2,
                z_range: z1..=z2,
            }
        })
        .collect()
}

fn discard_instructions_exceeding_range(
    instructions: &mut Vec<Instruction>,
    min_value: i32,
    max_value: i32,
) {
    instructions.retain(|instruction| {
        let ranges = [
            &instruction.x_range,
            &instruction.y_range,
            &instruction.z_range,
        ];
        ranges
            .iter()
            .all(|range| *range.start() >= min_value && *range.end() <= max_value)
    });
}

fn calculate_how_many_cubes_are_on_after_initialization(instructions: &[Instruction]) -> usize {
    let mut on_cubes: HashSet<[i32; 3]> = HashSet::new();

    for instruction in instructions {
        for x in *instruction.x_range.start()..=*instruction.x_range.end() {
            for y in *instruction.y_range.start()..=*instruction.y_range.end() {
                for z in *instruction.z_range.start()..=*instruction.z_range.end() {
                    match instruction.is_on {
                        true => on_cubes.insert([x, y, z]),
                        false => on_cubes.remove(&[x, y, z]),
                    };
                }
            }
        }
    }

    on_cubes.len()
}

pub fn solve() -> String {
    let mut instructions = process_data("./input/22.txt");
    discard_instructions_exceeding_range(&mut instructions, -50, 50);
    let result = calculate_how_many_cubes_are_on_after_initialization(&instructions);
    format!("Day 22: Reactor Reboot (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_22_reactor_reboot::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let mut instructions = process_data("./test_input/22_1.txt");
        discard_instructions_exceeding_range(&mut instructions, -50, 50);
        assert_eq!(instructions.len(), 20);
        let result = calculate_how_many_cubes_are_on_after_initialization(&instructions);
        assert_eq!(result, 590784);
    }
}
