use std::collections::{HashMap, HashSet};

struct Scanner {
    beacons: Vec<[i16; 3]>,
    pairwise_distances_map: HashMap<[i16; 3], ([i16; 3], [i16; 3])>,
    pairwise_distances: HashSet<[i16; 3]>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            beacons: Vec::new(),
            pairwise_distances_map: HashMap::new(),
            pairwise_distances: HashSet::new(),
        }
    }

    fn add_beacon(&mut self, point: [i16; 3]) {
        self.beacons.push(point);
    }

    fn calculate_pairwise_distances(&mut self) {
        for (index, [x, y, z]) in self.beacons.iter().enumerate() {
            for [other_x, other_y, other_z] in &self.beacons[index + 1..] {
                let mut distance = [
                    (x - other_x).abs(),
                    (y - other_y).abs(),
                    (z - other_z).abs(),
                ];
                distance.sort_unstable();
                self.pairwise_distances_map
                    .insert(distance, ([*x, *y, *z], [*other_x, *other_y, *other_z]));
                self.pairwise_distances.insert(distance);
            }
        }
    }
}

fn process_data(path: &str) -> Vec<Scanner> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content.lines().filter(|line| !line.is_empty()).fold(
        Vec::new(),
        |mut scanners: Vec<Scanner>, line| {
            if line.starts_with("---") {
                scanners.push(Scanner::new());
            } else {
                let [x, y, z] = line
                    .split(',')
                    .map(|number| number.parse::<i16>().expect("Should be a valid i16 value"))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Should be three valid i16 values");
                scanners.last_mut().unwrap().add_beacon([x, y, z]);
            }
            scanners
        },
    )
}

fn find_matching_scanners(scanner: &Scanner, other_scanner: &Scanner) -> bool {
    let intersection: HashSet<_> = scanner
        .pairwise_distances
        .intersection(&other_scanner.pairwise_distances)
        .clone()
        .collect::<HashSet<_>>();

    intersection.len() >= 66
}

fn find_matching_beacons_pairs(
    scanner: &Scanner,
    other_scanner: &Scanner,
) -> Vec<([i16; 3], [i16; 3])> {
    let mut matching_pair_distances = Vec::new();

    for (distance_value, point_pair) in &scanner.pairwise_distances_map {
        if let Some(other_point_pair) = other_scanner.pairwise_distances_map.get(distance_value) {
            matching_pair_distances.push((*point_pair, *other_point_pair));
        }
    }

    let mut real_pairs_counts: HashMap<[i16; 3], HashMap<[i16; 3], usize>> = HashMap::new();

    for ((pair_1, pair_2), (pair_3, pair_4)) in matching_pair_distances {
        let update_count =
            |map: &mut HashMap<[i16; 3], HashMap<[i16; 3], usize>>, p1: [i16; 3], p2: [i16; 3]| {
                map.entry(p1)
                    .or_default()
                    .entry(p2)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            };

        update_count(&mut real_pairs_counts, pair_1, pair_3);
        update_count(&mut real_pairs_counts, pair_1, pair_4);
        update_count(&mut real_pairs_counts, pair_2, pair_3);
        update_count(&mut real_pairs_counts, pair_2, pair_4);
    }

    let mut real_pairs = Vec::new();

    for (point, counts) in real_pairs_counts {
        if let Some((best_match, _count)) = counts.iter().max_by_key(|&(_, count)| count) {
            real_pairs.push((point, *best_match));
        }
    }

    real_pairs
}

fn determine_rotation(beacon_pairs: &[([i16; 3], [i16; 3])]) -> fn([i16; 3]) -> [i16; 3] {
    let (point_a, other_point_a) = beacon_pairs[0];
    let (point_b, other_point_b) = beacon_pairs[1];
    let dist_a_b = [
        point_a[0] - point_b[0],
        point_a[1] - point_b[1],
        point_a[2] - point_b[2],
    ];

    let other_dist_a_b = [
        other_point_a[0] - other_point_b[0],
        other_point_a[1] - other_point_b[1],
        other_point_a[2] - other_point_b[2],
    ];

    let rotations: [fn([i16; 3]) -> [i16; 3]; 24] = [
        |[x, y, z]| [x, y, z],
        |[x, y, z]| [x, -y, -z],
        |[x, y, z]| [-x, -y, z],
        |[x, y, z]| [-x, y, -z],
        |[x, y, z]| [x, -z, y],
        |[x, y, z]| [x, z, -y],
        |[x, y, z]| [-x, z, y],
        |[x, y, z]| [-x, -z, -y],
        |[x, y, z]| [y, -x, z],
        |[x, y, z]| [y, x, -z],
        |[x, y, z]| [-y, x, z],
        |[x, y, z]| [-y, -x, -z],
        |[x, y, z]| [y, z, x],
        |[x, y, z]| [y, -z, -x],
        |[x, y, z]| [-y, -z, x],
        |[x, y, z]| [-y, z, -x],
        |[x, y, z]| [z, x, y],
        |[x, y, z]| [z, -x, -y],
        |[x, y, z]| [-z, -x, y],
        |[x, y, z]| [-z, x, -y],
        |[x, y, z]| [z, -y, x],
        |[x, y, z]| [z, y, -x],
        |[x, y, z]| [-z, y, x],
        |[x, y, z]| [-z, -y, -x],
    ];

    for rotation in rotations {
        let rotated_distance = rotation(other_dist_a_b);
        if dist_a_b == rotated_distance {
            return rotation;
        }
    }

    unreachable!()
}

fn determine_displacement(
    beacon_pairs: &[([i16; 3], [i16; 3])],
    rotation: fn([i16; 3]) -> [i16; 3],
) -> impl Fn([i16; 3]) -> [i16; 3] {
    let ([x, y, z], [other_x, other_y, other_z]) = beacon_pairs[0];
    let [rot_other_x, rot_other_y, rot_other_z] = rotation([other_x, other_y, other_z]);
    let [dx, dy, dz] = [x - rot_other_x, y - rot_other_y, z - rot_other_z];
    let displacement = move |[x, y, z]: [i16; 3]| -> [i16; 3] { [x + dx, y + dy, z + dz] };

    let (point, other_point) = beacon_pairs[1];
    let rotated_other_point = rotation(other_point);
    assert_eq!(point, displacement(rotated_other_point));

    displacement
}

fn rotate_and_align_beacons(
    scanner: &Scanner,
    rotate: fn([i16; 3]) -> [i16; 3],
    align: impl Fn([i16; 3]) -> [i16; 3],
) -> Vec<[i16; 3]> {
    scanner
        .beacons
        .iter()
        .map(|beacon| align(rotate(*beacon)))
        .collect()
}

fn count_beacons(scanners: &mut Vec<Scanner>) -> usize {
    for scanner in &mut *scanners {
        scanner.calculate_pairwise_distances()
    }

    while scanners.len() > 1 {
        for index in 1..scanners.len() {
            let is_match = find_matching_scanners(&scanners[0], &scanners[index]);
            if is_match {
                let matching_beacons_pairs =
                    find_matching_beacons_pairs(&scanners[0], &scanners[index]);
                let rotation = determine_rotation(&matching_beacons_pairs);
                let displacement = determine_displacement(&matching_beacons_pairs, rotation);
                let aligned_beacons =
                    rotate_and_align_beacons(&scanners[index], rotation, displacement);

                scanners[0].beacons.extend(aligned_beacons);
                scanners[0].beacons.sort_unstable();
                scanners[0].beacons.dedup();
                scanners[0].calculate_pairwise_distances();
                scanners.remove(index);
                break;
            }
        }
    }

    scanners[0].beacons.len()
}

pub fn solve() -> String {
    let mut scanners_data = process_data("./input/19.txt");
    let result = count_beacons(&mut scanners_data);
    format!("Day 19: Beacon Scanner (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_19_beacon_scanner::part_1::*;

    #[test]
    fn read_data_from_file_and_convert_to_vector() {
        let mut scanners_data = process_data("./test_input/19.txt");
        assert_eq!(scanners_data.len(), 5);
        assert_eq!(scanners_data[0].beacons.len(), 25);
        scanners_data[1].calculate_pairwise_distances();
        assert_eq!(scanners_data[1].pairwise_distances.len(), 300);
        let result = count_beacons(&mut scanners_data);
        assert_eq!(result, 79);
    }
}
