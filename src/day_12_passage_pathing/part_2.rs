#[derive(PartialEq)]
enum CaveKind {
    Small,
    Big,
}

struct Cave {
    name: String,
    id: usize,
    kind: CaveKind,
    neighbours: Vec<usize>,
}

impl Cave {
    fn new(name: &str, id: usize) -> Self {
        Self {
            name: name.to_owned(),
            id,
            kind: match name.chars().all(char::is_uppercase) {
                false => CaveKind::Small,
                true => CaveKind::Big,
            },
            neighbours: Vec::new(),
        }
    }
}

fn get_id_or_create_and_add_cave(name: &str, caves: &mut Vec<Cave>) -> usize {
    match caves.iter().find(|cave| cave.name == name) {
        Some(cave) => cave.id,
        None => {
            let next_id = caves.len();
            caves.push(Cave::new(name, next_id));
            next_id
        }
    }
}

fn process_data(path: &str) -> Vec<Cave> {
    let file_content = std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand");
    let mut caves: Vec<Cave> = Vec::new();

    file_content.lines().for_each(|line| {
        if let Some((cave_from_name, cave_to_name)) = line.split_once('-') {
            let cave_from_id = get_id_or_create_and_add_cave(cave_from_name, &mut caves);
            let cave_to_id = get_id_or_create_and_add_cave(cave_to_name, &mut caves);
            caves[cave_from_id].neighbours.push(cave_to_id);
            caves[cave_to_id].neighbours.push(cave_from_id);
        }
    });

    caves
}

fn count_paths(
    current_cave_id: &usize,
    start: &usize,
    end: &usize,
    visited: &mut Vec<usize>,
    caves: &Vec<Cave>,
    mut small_visited_twice: bool,
) -> usize {
    if current_cave_id == end {
        return 1;
    }

    if caves[*current_cave_id].kind == CaveKind::Small && visited.contains(current_cave_id) {
        if small_visited_twice || current_cave_id == start {
            return 0;
        }

        small_visited_twice = true;
    }

    visited.push(*current_cave_id);

    let path_counter: usize = caves[*current_cave_id]
        .neighbours
        .iter()
        .map(|cave_id| count_paths(cave_id, start, end, visited, caves, small_visited_twice))
        .sum();

    visited.pop();
    path_counter
}

pub fn solve() -> String {
    let caves = process_data("./input/12.txt");
    let mut visited: Vec<usize> = Vec::new();
    let start_id = &caves.iter().find(|cave| cave.name == "start").unwrap().id;
    let end_id = &caves.iter().find(|cave| cave.name == "end").unwrap().id;
    let result = count_paths(start_id, start_id, end_id, &mut visited, &caves, false);
    format!("Day 12: Passage Pathing (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_12_passage_pathing::part_2::*;

    #[test]
    fn solve_with_test_data() {
        let caves = process_data("./test_input/12.txt");
        assert_eq!(caves.len(), 6);
        let mut visited: Vec<usize> = Vec::new();
        let start_id = &caves.iter().find(|cave| cave.name == "start").unwrap().id;
        assert_eq!(*start_id, 0);
        let end_id = &caves.iter().find(|cave| cave.name == "end").unwrap().id;
        assert_eq!(*end_id, 5);
        let result = count_paths(start_id, start_id, end_id, &mut visited, &caves, false);
        assert_eq!(result, 36);
    }
}
