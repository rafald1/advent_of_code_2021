use std::ops::RangeInclusive;

#[derive(Clone)]
struct Cuboid {
    x_rng: RangeInclusive<i64>,
    y_rng: RangeInclusive<i64>,
    z_rng: RangeInclusive<i64>,
}

impl Cuboid {
    fn calculate_volume(&self) -> i64 {
        (self.x_rng.end() - self.x_rng.start() + 1)
            * (self.y_rng.end() - self.y_rng.start() + 1)
            * (self.z_rng.end() - self.z_rng.start() + 1)
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let cuboid = Self {
            x_rng: *self.x_rng.start().max(other.x_rng.start())
                ..=*self.x_rng.end().min(other.x_rng.end()),
            y_rng: *self.y_rng.start().max(other.y_rng.start())
                ..=*self.y_rng.end().min(other.y_rng.end()),
            z_rng: *self.z_rng.start().max(other.z_rng.start())
                ..=*self.z_rng.end().min(other.z_rng.end()),
        };

        (cuboid.x_rng.start() <= cuboid.x_rng.end()
            && cuboid.y_rng.start() <= cuboid.y_rng.end()
            && cuboid.z_rng.start() <= cuboid.z_rng.end())
        .then_some(cuboid)
    }
}

#[derive(Clone)]
struct Instruction {
    is_on: bool,
    cuboid: Cuboid,
}

fn process_data(path: &str) -> Vec<Instruction> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .map(|line| {
            let (is_on, rest) = line.split_once(' ').unwrap();
            let ranges: Vec<i64> = rest
                .split(',')
                .flat_map(|range| range[2..].split("..").map(|num| num.parse().unwrap()))
                .collect();
            let [x1, x2, y1, y2, z1, z2]: [i64; 6] = ranges.try_into().unwrap();
            Instruction {
                is_on: is_on == "on",
                cuboid: Cuboid {
                    x_rng: x1..=x2,
                    y_rng: y1..=y2,
                    z_rng: z1..=z2,
                },
            }
        })
        .collect()
}

fn calculate_how_many_cubes_are_on_after_initialization(instructions: &[Instruction]) -> i64 {
    let mut processed_instructions: Vec<Instruction> = Vec::new();

    for instruction in instructions {
        let mut new_processed_instructions = Vec::new();

        if instruction.is_on {
            new_processed_instructions.push(instruction.clone());
        }

        for processed_instruction in processed_instructions.iter() {
            if let Some(intersection) = instruction.cuboid.intersect(&processed_instruction.cuboid)
            {
                new_processed_instructions.push(Instruction {
                    is_on: !processed_instruction.is_on,
                    cuboid: intersection,
                });
            }
        }

        processed_instructions.extend(new_processed_instructions);
    }

    processed_instructions
        .iter()
        .map(|processed_instruction| match processed_instruction.is_on {
            true => processed_instruction.cuboid.calculate_volume(),
            false => -processed_instruction.cuboid.calculate_volume(),
        })
        .sum::<i64>()
}

pub fn solve() -> String {
    let instructions = process_data("./input/22.txt");
    let result = calculate_how_many_cubes_are_on_after_initialization(&instructions);
    format!("Day 22: Reactor Reboot (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_22_reactor_reboot::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let instructions = process_data("./test_input/22_2.txt");
        assert_eq!(instructions.len(), 60);
        let result = calculate_how_many_cubes_are_on_after_initialization(&instructions);
        assert_eq!(result, 2758514936282235);
    }
}
