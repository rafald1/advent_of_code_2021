#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Point(usize, usize);

enum Instruction {
    Vertical(usize),
    Horizontal(usize),
}

fn process_data(path: &str) -> (Vec<Point>, Vec<Instruction>) {
    let file_content = std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand");

    let mut lines = file_content.lines();
    let mut transparent_paper: Vec<Point> = Vec::new();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (x, y) = line.split_once(',').unwrap();
        transparent_paper.push(Point(x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut fold_instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let (text, value) = line.split_once('=').unwrap();
        let value: usize = value.parse().unwrap();

        let fold_instruction = match text.chars().last().unwrap() {
            'x' => Instruction::Vertical(value),
            'y' => Instruction::Horizontal(value),
            _ => unreachable!(),
        };

        fold_instructions.push(fold_instruction);
    }

    (transparent_paper, fold_instructions)
}

fn fold(mut paper: Vec<Point>, instructions: &[Instruction]) -> Vec<Point> {
    for instruction in instructions {
        paper = paper
            .into_iter()
            .map(|Point(x, y)| match instruction {
                Instruction::Vertical(col) => Point(if x < *col { x } else { 2 * col - x }, y),
                Instruction::Horizontal(row) => Point(x, if y < *row { y } else { 2 * row - y }),
            })
            .collect::<Vec<_>>();
    }

    paper.sort();
    paper.dedup();
    paper
}

fn show_letters(paper: &[Point]) -> String {
    let max_x = paper.iter().map(|Point(x, _)| x).max().unwrap();
    let max_y = paper.iter().map(|Point(_, y)| y).max().unwrap();
    let mut output: String = String::new();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if paper.contains(&Point(x, y)) {
                output.push('█');
            } else {
                output.push(' ');
            }
        }

        output.push('\n');
    }

    output
}

pub fn solve() -> String {
    let (paper, instructions) = process_data("./input/13.txt");
    let folded_paper = fold(paper, &instructions);
    let result = show_letters(&folded_paper);
    format!("Day 13: Transparent Origami (Part 2) answer: \n{}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_13_transparent_origami::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let (paper, instructions) = process_data("./test_input/13.txt");
        assert_eq!(paper.len(), 18);
        let folded_paper = fold(paper, &instructions);
        assert_eq!(folded_paper.len(), 16);
    }
}
