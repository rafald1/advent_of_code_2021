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

fn calculate_highest_possible_y_position(_x1: i32, _x2: i32, y1: i32, y2: i32) -> i32 {
    let y_min = y1.min(y2);
    -(y_min + 1) * (-(y_min + 1) + 1) / 2
}

pub fn solve() -> String {
    let [x1, x2, y1, y2] = process_data("./input/17.txt");
    let result = calculate_highest_possible_y_position(x1, x2, y1, y2);
    format!("Day 17: Trick Shot (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_17_trick_shot::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let [x1, x2, y1, y2] = process_data("./test_input/17.txt");
        assert_eq!([x1, x2, y1, y2], [20, 30, -10, -5]);
        let result = calculate_highest_possible_y_position(x1, x2, y1, y2);
        assert_eq!(result, 45);
    }
}
