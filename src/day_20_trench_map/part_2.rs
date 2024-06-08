struct ImgState;

impl ImgState {
    const UNSEEN_PIXELS_ARE_LIT: bool = true;
    const UNSEEN_PIXELS_ARE_UNLIT: bool = false;
}

struct Image {
    pixels: Vec<Vec<bool>>,
    state: bool,
}

fn process_data(path: &str) -> ([bool; 512], Vec<Vec<bool>>) {
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

    let pixels: Vec<Vec<bool>> = lines
        .skip(1)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (algorithm, pixels)
}

fn calculate_index_of_enhancement_algorithm(&[x, y]: &[i32; 2], img: &Image) -> usize {
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
        let [nx, ny] = [x + dx, y + dy];

        if nx < 0 || nx >= img.pixels[0].len() as i32 || ny < 0 || ny >= img.pixels.len() as i32 {
            number |= (img.state as usize) << (8 - index);
        } else {
            number |= (img.pixels[ny as usize][nx as usize] as usize) << (8 - index);
        }
    }

    number
}

fn enhance_image(img: &mut Image, algorithm: &[bool; 512], num_of_rounds: usize) -> usize {
    for _ in 0..num_of_rounds {
        let mut new_pixels = Vec::new();

        for y in -1..img.pixels.len() as i32 + 1 {
            let row: Vec<bool> = (-1..img.pixels[0].len() as i32 + 1)
                .map(|x| algorithm[calculate_index_of_enhancement_algorithm(&[x, y], img)])
                .collect();
            new_pixels.push(row);
        }

        let new_state = match img.state {
            ImgState::UNSEEN_PIXELS_ARE_LIT => algorithm[511],
            ImgState::UNSEEN_PIXELS_ARE_UNLIT => algorithm[0],
        };

        *img = Image {
            pixels: new_pixels,
            state: new_state,
        }
    }

    img.pixels.iter().flatten().map(|&pxl| pxl as usize).sum()
}

pub fn solve() -> String {
    let (algorithm, pixels) = process_data("./input/20.txt");
    let mut image = Image {
        pixels,
        state: ImgState::UNSEEN_PIXELS_ARE_UNLIT,
    };
    let result = enhance_image(&mut image, &algorithm, 50);
    format!("Day 20: Trench Map (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_20_trench_map::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let (algorithm, pixels) = process_data("./test_input/20.txt");
        assert_eq!(algorithm.len(), 512);
        assert_eq!(pixels.len(), 5);
        assert_eq!(pixels[0].len(), 5);
        let mut image = Image {
            pixels,
            state: ImgState::UNSEEN_PIXELS_ARE_UNLIT,
        };
        let result = enhance_image(&mut image, &algorithm, 50);
        assert_eq!(result, 3351);
    }
}
