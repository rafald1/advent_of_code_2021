use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn process_data(path: &str) -> Vec<Vec<i8>> {
    let file_content = std::fs::read_to_string(path).unwrap();
    let cavern: Vec<Vec<i8>> = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should be 0-9 number") as i8)
                .collect()
        })
        .collect();

    let (width, height) = (cavern[0].len(), cavern.len());

    (0..height * 5)
        .map(|y| {
            (0..width * 5)
                .map(|x| {
                    (cavern[y % height][x % width] + (x / width + y / height) as i8 - 1) % 9 + 1
                })
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

fn find_the_shortest_path(
    cavern: Vec<Vec<i8>>,
    start: (usize, usize),
    end: (usize, usize),
) -> isize {
    let width = cavern[0].len();
    let height = cavern.len();
    let mut min_distances: Vec<Vec<isize>> = vec![vec![isize::MAX; width]; height];
    min_distances[start.1][start.0] = 0;
    let mut p_queue: BinaryHeap<(Reverse<isize>, (usize, usize))> = BinaryHeap::new();
    p_queue.push((Reverse(0), start));

    while let Some((Reverse(min_distance), (x, y))) = p_queue.pop() {
        if (x, y) == end {
            return min_distances[end.1][end.0];
        }

        for (nx, ny) in get_valid_neighbours(x, y, width, height) {
            let new_min_distance: isize = min_distance + cavern[ny][nx] as isize;

            if min_distances[ny][nx] > new_min_distance {
                min_distances[ny][nx] = new_min_distance;
                p_queue.push((Reverse(new_min_distance), (nx, ny)));
            }
        }
    }

    unreachable!()
}

pub fn solve() -> String {
    let cavern = process_data("./input/15.txt");
    let start: (usize, usize) = (0, 0);
    let end = (cavern.len() - 1, cavern[0].len() - 1);
    let result = find_the_shortest_path(cavern, start, end);
    format!("Day 15: Chiton (Part 2) answer: {:?}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_15_chiton::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let cavern = process_data("./test_input/15.txt");
        assert_eq!(cavern.len(), 50);
        let start: (usize, usize) = (0, 0);
        let end = (cavern.len() - 1, cavern[0].len() - 1);
        let result = find_the_shortest_path(cavern, start, end);
        assert_eq!(result, 315);
    }
}
