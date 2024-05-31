use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Point(usize, usize);

enum Instruction {
    Vertical(usize),
    Horizontal(usize),
}

fn process_data(path: &str) -> (HashSet<Point>, Instruction) {
    let file_content = std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand");

    let mut lines = file_content.lines();
    let mut transparent_paper: HashSet<Point> = HashSet::new();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (x, y) = line.split_once(',').unwrap();
        transparent_paper.insert(Point(x.parse().unwrap(), y.parse().unwrap()));
    }

    let (text, value) = lines.next().unwrap().split_once('=').unwrap();
    let value: usize = value.parse().unwrap();

    let fold_instruction = match text.chars().last().unwrap() {
        'x' => Instruction::Vertical(value),
        'y' => Instruction::Horizontal(value),
        _ => unreachable!(),
    };

    (transparent_paper, fold_instruction)
}

fn fold_once(paper: HashSet<Point>, instruction: Instruction) -> usize {
    paper
        .into_iter()
        .map(|Point(x, y)| match instruction {
            Instruction::Vertical(col) => Point(if x < col { x } else { 2 * col - x }, y),
            Instruction::Horizontal(row) => Point(x, if y < row { y } else { 2 * row - y }),
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn solve() -> String {
    let (paper, instruction) = process_data("./input/13.txt");
    let result = fold_once(paper, instruction);
    format!("Day 13: Transparent Origami (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_13_transparent_origami::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (paper, instruction) = process_data("./test_input/13.txt");
        assert_eq!(paper.len(), 18);
        let result = fold_once(paper, instruction);
        assert_eq!(result, 17);
    }
}
