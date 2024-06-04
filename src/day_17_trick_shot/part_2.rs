fn process_data(path: &str) -> [i32; 4] {
    std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand")
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .map(|(x_values, y_values)| {
            let (x1, x2) = x_values.split_once("..").expect("Should contain x range");
            let (y1, y2) = y_values.split_once("..").expect("Should contain y range");
            [x1, x2, y1, y2].map(|value| {
                value
                    .parse::<i32>()
                    .expect("Should contain valid i32 values")
            })
        })
        .expect("Should be exactly four i32 values")
}

fn find_valid_velocity_x_values(x1: i32, x2: i32) -> Vec<i32> {
    let mut valid_velocity_x_values = Vec::new();

    for initial_velocity_x in 1..=x2 {
        let mut current_position_x = 0;
        let mut velocity_x = initial_velocity_x;

        while velocity_x > 0 && current_position_x <= x2 {
            current_position_x += velocity_x;
            velocity_x -= 1;

            if current_position_x >= x1 && current_position_x <= x2 {
                valid_velocity_x_values.push(initial_velocity_x);
                break;
            }
        }
    }

    valid_velocity_x_values
}

fn find_all_valid_velocity_pairs(x1: i32, x2: i32, y1: i32, y2: i32) -> usize {
    let valid_velocity_x_values = find_valid_velocity_x_values(x1, x2);
    let y_min = y1.min(y2);
    let y_max = y1.max(y2);
    let mut counter: usize = 0;

    for velocity_x in valid_velocity_x_values {
        for velocity_y in y_min..=-(y_min + 1) {
            let mut current_position_x = velocity_x;
            let mut current_position_y = velocity_y;

            for turn in 1.. {
                if current_position_x > x2 || current_position_y < y_min {
                    break;
                }

                if current_position_x >= x1
                    && current_position_x <= x2
                    && current_position_y <= y_max
                    && current_position_y >= y_min
                {
                    counter += 1;
                    break;
                }

                current_position_x += 0.max(velocity_x - turn);
                current_position_y += velocity_y - turn;
            }
        }
    }

    counter
}

pub fn solve() -> String {
    let [x1, x2, y1, y2] = process_data("./input/17.txt");
    let result = find_all_valid_velocity_pairs(x1, x2, y1, y2);
    format!("Day 17: Trick Shot (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_17_trick_shot::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let [x1, x2, y1, y2] = process_data("./test_input/17.txt");
        assert_eq!([x1, x2, y1, y2], [20, 30, -10, -5]);
        let result = find_all_valid_velocity_pairs(x1, x2, y1, y2);
        assert_eq!(result, 112);
    }
}
