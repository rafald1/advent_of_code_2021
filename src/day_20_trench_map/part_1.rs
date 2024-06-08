use std::collections::HashSet;

struct ImgState;

impl ImgState {
    const UNSEEN_PIXELS_ARE_LIT: bool = true;
    const UNSEEN_PIXELS_ARE_UNLIT: bool = false;
}

struct Image {
    lit_pixels: HashSet<[i16; 2]>,
    min_x: i16,
    max_x: i16,
    min_y: i16,
    max_y: i16,
    state: bool,
}

impl Image {
    fn new(lit_pixels: HashSet<[i16; 2]>, state: bool) -> Self {
        let [min_x, min_y, max_x, max_y] = lit_pixels.iter().fold(
            [i16::MAX, i16::MAX, i16::MIN, i16::MIN],
            |[min_x, min_y, max_x, max_y], &[x, y]| {
                [min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y)]
            },
        );

        Self {
            lit_pixels,
            min_x,
            max_x,
            min_y,
            max_y,
            state,
        }
    }
}

fn process_data(path: &str) -> ([bool; 512], HashSet<[i16; 2]>) {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut lines = file_content.lines();

    let algorithm: [bool; 512] = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("There should be exactly 512 values");

    lines.next();

    let mut lit_pixels: HashSet<[i16; 2]> = HashSet::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                lit_pixels.insert([x as i16, y as i16]);
            }
        }
    }

    (algorithm, lit_pixels)
}

fn calculate_index_of_enhancement_algorithm(&[x, y]: &[i16; 2], img: &Image) -> usize {
    let section = [
        [-1, -1],
        [0, -1],
        [1, -1],
        [-1, 0],
        [0, 0],
        [1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
    ];

    let mut number: usize = 0;

    for (index, &[dx, dy]) in section.iter().enumerate() {
        if x + dx < img.min_x || x + dx > img.max_x || y + dy < img.min_y || y + dy > img.max_y {
            number |= (img.state as usize) << (8 - index);
        } else {
            number |= if img.lit_pixels.contains(&[x + dx, y + dy]) {
                1
            } else {
                0
            } << (8 - index);
        }
    }

    number
}

fn enhance_image(img: &mut Image, algorithm: &[bool; 512]) -> usize {
    for _round in 0..2 {
        let mut new_lit_pixels: HashSet<[i16; 2]> = HashSet::new();

        for x in img.min_x - 1..=img.max_x + 1 {
            for y in img.min_y - 1..=img.max_y + 1 {
                let index = calculate_index_of_enhancement_algorithm(&[x, y], img);
                if algorithm[index] {
                    new_lit_pixels.insert([x, y]);
                }
            }
        }

        let new_state = match img.state {
            ImgState::UNSEEN_PIXELS_ARE_LIT => algorithm[511],
            ImgState::UNSEEN_PIXELS_ARE_UNLIT => algorithm[0],
        };

        *img = Image::new(new_lit_pixels, new_state);
    }

    img.lit_pixels.len()
}

pub fn solve() -> String {
    let (algorithm, lit_pixels) = process_data("./input/20.txt");
    let mut image = Image::new(lit_pixels, ImgState::UNSEEN_PIXELS_ARE_UNLIT);
    let result = enhance_image(&mut image, &algorithm);
    format!("Day 20: Trench Map (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_20_trench_map::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let (algorithm, lit_pixels) = process_data("./test_input/20.txt");
        assert_eq!(algorithm.len(), 512);
        assert_eq!(lit_pixels.len(), 10);
        let mut image = Image::new(lit_pixels, ImgState::UNSEEN_PIXELS_ARE_UNLIT);
        let result = enhance_image(&mut image, &algorithm);
        assert_eq!(result, 35);
    }
}
