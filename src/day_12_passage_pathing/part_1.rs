use std::collections::HashMap;

fn process_data(path: &str) -> HashMap<String, Vec<String>> {
    let file_content = std::fs::read_to_string(path)
        .expect("The input file should be placed in the input folder beforehand");
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    file_content.lines().for_each(|line| {
        if let Some((from, to)) = line.split_once('-') {
            edges
                .entry(from.to_owned())
                .or_default()
                .push(to.to_owned());
            edges
                .entry(to.to_owned())
                .or_default()
                .push(from.to_owned());
        }
    });

    edges
}

fn count_paths<'a>(
    current_vertex: &'a str,
    visited: &mut Vec<&'a str>,
    edges: &'a HashMap<String, Vec<String>>,
) -> usize {
    visited.push(current_vertex);

    if current_vertex == "end" {
        return 1;
    }

    let mut path_counter: usize = 0;

    for neighbour_vertex in &edges[current_vertex] {
        if neighbour_vertex.chars().all(char::is_lowercase)
            && visited.contains(&&neighbour_vertex[..])
        {
            continue;
        }

        path_counter += count_paths(neighbour_vertex, visited, edges);
        visited.pop();
    }

    path_counter
}

pub fn solve() -> String {
    let graph_edges: HashMap<String, Vec<String>> = process_data("./input/12.txt");
    let mut visited: Vec<&str> = Vec::new();
    let result = count_paths("start", &mut visited, &graph_edges);
    format!("Day 12: Passage Pathing (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_12_passage_pathing::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let graph_edges: HashMap<String, Vec<String>> = process_data("./test_input/12.txt");
        assert_eq!(graph_edges.len(), 6);
        let mut visited: Vec<&str> = Vec::new();
        let result = count_paths("start", &mut visited, &graph_edges);
        assert_eq!(result, 10);
    }
}
