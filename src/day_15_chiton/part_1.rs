use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn process_data(path: &str) -> Vec<Vec<u8>> {
    let file_content = std::fs::read_to_string(path).unwrap();
    file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should be 0-9 number") as u8)
                .collect()
        })
        .collect()
}

fn get_valid_neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    Vec::from([
        (x, y.wrapping_sub(1)),
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
    ])
    .into_iter()
    .filter(|&(nx, ny)| nx < width && ny < height)
    .collect()
}

fn find_the_shortest_path(cavern: Vec<Vec<u8>>) -> usize {
    let start = (0, 0);
    let width = cavern[0].len();
    let height = cavern.len();
    let end = (width - 1, height - 1);

    let mut min_distances: Vec<Vec<usize>> = vec![vec![usize::MAX; width]; height];
    min_distances[start.1][start.0] = 0;
    let mut p_queue: BinaryHeap<(Reverse<isize>, (usize, usize))> = BinaryHeap::new();
    p_queue.push((Reverse(0), start));

    while let Some((Reverse(min_distance), (x, y))) = p_queue.pop() {
        if (x, y) == end {
            return min_distances[end.1][end.0];
        }

        for (nx, ny) in get_valid_neighbours(x, y, width, height) {
            let new_min_distance: isize = min_distance + cavern[ny][nx] as isize;
            if min_distances[ny][nx] > new_min_distance as usize {
                min_distances[ny][nx] = new_min_distance as usize;
                p_queue.push((Reverse(new_min_distance), (nx, ny)));
            }
        }
    }

    unreachable!()
}

pub fn solve() -> String {
    let cavern = process_data("./input/15.txt");
    let result = find_the_shortest_path(cavern);
    format!("Day 15: Chiton (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_15_chiton::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let cavern = process_data("./test_input/15.txt");
        assert_eq!(cavern.len(), 10);
        let result = find_the_shortest_path(cavern);
        assert_eq!(result, 40);
    }
}
