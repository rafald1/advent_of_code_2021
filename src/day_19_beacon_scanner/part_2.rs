use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Point3D {
    x: i16,
    y: i16,
    z: i16,
}

impl Point3D {
    fn from(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    fn calculate_distance(&self, other: &Self) -> [i16; 3] {
        [
            (self.x - other.x).abs(),
            (self.y - other.y).abs(),
            (self.z - other.z).abs(),
        ]
    }
}

struct Scanner {
    position: Option<Point3D>,
    beacons: Vec<Point3D>,
    pairwise_distances_map: HashMap<[i16; 3], (Point3D, Point3D)>,
    pairwise_distances: HashSet<[i16; 3]>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            position: None,
            beacons: Vec::new(),
            pairwise_distances_map: HashMap::new(),
            pairwise_distances: HashSet::new(),
        }
    }

    fn add_beacon(&mut self, point: Point3D) {
        self.beacons.push(point);
    }

    fn calculate_pairwise_distances(&mut self) {
        for (index, beacon) in self.beacons.iter().enumerate() {
            for other_beacon in &self.beacons[index + 1..] {
                let mut distance = beacon.calculate_distance(other_beacon);
                distance.sort_unstable();
                self.pairwise_distances_map
                    .insert(distance, (*beacon, *other_beacon));
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
                scanners.last_mut().unwrap().add_beacon(Point3D { x, y, z });
            }
            scanners
        },
    )
}

fn find_matching_scanners(scanner: &Scanner, other_scanner: &Scanner) -> bool {
    let intersection: HashSet<_> = scanner
        .pairwise_distances
        .intersection(&other_scanner.pairwise_distances)
        .collect::<HashSet<_>>();

    intersection.len() >= 66
}

fn find_matching_beacons_pairs(
    scanner: &Scanner,
    other_scanner: &Scanner,
) -> Vec<(Point3D, Point3D)> {
    let mut matching_pair_distances = Vec::new();

    for (distance_value, point_pair) in &scanner.pairwise_distances_map {
        if let Some(other_point_pair) = other_scanner.pairwise_distances_map.get(distance_value) {
            matching_pair_distances.push((*point_pair, *other_point_pair));
        }
    }

    let mut real_pairs_counts: HashMap<Point3D, HashMap<Point3D, usize>> = HashMap::new();

    for ((pair_1, pair_2), (pair_3, pair_4)) in matching_pair_distances {
        let update_count =
            |map: &mut HashMap<Point3D, HashMap<Point3D, usize>>, p1: Point3D, p2: Point3D| {
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

fn determine_rotation(beacon_pairs: &[(Point3D, Point3D)]) -> fn(Point3D) -> Point3D {
    let (point_a, other_point_a) = beacon_pairs[0];
    let (point_b, other_point_b) = beacon_pairs[1];
    let dist_a_b = Point3D::from(
        point_a.x - point_b.x,
        point_a.y - point_b.y,
        point_a.z - point_b.z,
    );

    let other_dist_a_b = Point3D::from(
        other_point_a.x - other_point_b.x,
        other_point_a.y - other_point_b.y,
        other_point_a.z - other_point_b.z,
    );

    let rotations: [fn(Point3D) -> Point3D; 24] = [
        |p: Point3D| Point3D::from(p.x, p.y, p.z),
        |p: Point3D| Point3D::from(p.x, -p.y, -p.z),
        |p: Point3D| Point3D::from(-p.x, -p.y, p.z),
        |p: Point3D| Point3D::from(-p.x, p.y, -p.z),
        |p: Point3D| Point3D::from(p.x, -p.z, p.y),
        |p: Point3D| Point3D::from(p.x, p.z, -p.y),
        |p: Point3D| Point3D::from(-p.x, p.z, p.y),
        |p: Point3D| Point3D::from(-p.x, -p.z, -p.y),
        |p: Point3D| Point3D::from(p.y, -p.x, p.z),
        |p: Point3D| Point3D::from(p.y, p.x, -p.z),
        |p: Point3D| Point3D::from(-p.y, p.x, p.z),
        |p: Point3D| Point3D::from(-p.y, -p.x, -p.z),
        |p: Point3D| Point3D::from(p.y, p.z, p.x),
        |p: Point3D| Point3D::from(p.y, -p.z, -p.x),
        |p: Point3D| Point3D::from(-p.y, -p.z, p.x),
        |p: Point3D| Point3D::from(-p.y, p.z, -p.x),
        |p: Point3D| Point3D::from(p.z, p.x, p.y),
        |p: Point3D| Point3D::from(p.z, -p.x, -p.y),
        |p: Point3D| Point3D::from(-p.z, -p.x, p.y),
        |p: Point3D| Point3D::from(-p.z, p.x, -p.y),
        |p: Point3D| Point3D::from(p.z, -p.y, p.x),
        |p: Point3D| Point3D::from(p.z, p.y, -p.x),
        |p: Point3D| Point3D::from(-p.z, p.y, p.x),
        |p: Point3D| Point3D::from(-p.z, -p.y, -p.x),
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
    beacon_pairs: &[(Point3D, Point3D)],
    rotation: fn(Point3D) -> Point3D,
    scanner: &mut Scanner,
) -> impl Fn(Point3D) -> Point3D {
    let (point_a, other_point_a) = beacon_pairs[0];
    let rotated_other_point_a = rotation(other_point_a);

    let dx = point_a.x - rotated_other_point_a.x;
    let dy = point_a.y - rotated_other_point_a.y;
    let dz = point_a.z - rotated_other_point_a.z;

    scanner.position = Some(Point3D::from(dx, dy, dz));

    let displacement = move |p: Point3D| Point3D {
        x: p.x + dx,
        y: p.y + dy,
        z: p.z + dz,
    };

    let (point_b, other_point_b) = beacon_pairs[1];
    let rotated_other_point_b = rotation(other_point_b);
    assert_eq!(point_b, displacement(rotated_other_point_b));

    displacement
}

fn rotate_and_align_beacons(
    scanner: &Scanner,
    rotate: fn(Point3D) -> Point3D,
    align: impl Fn(Point3D) -> Point3D,
) -> Vec<Point3D> {
    scanner
        .beacons
        .iter()
        .map(|beacon| align(rotate(*beacon)))
        .collect()
}

fn find_maximum_distance_between_scanners(scanners: &mut Vec<Scanner>) -> i16 {
    for scanner in &mut *scanners {
        scanner.calculate_pairwise_distances()
    }

    scanners[0].position = Some(Point3D::from(0, 0, 0));

    let mut scanner_ids: Vec<usize> = (0..scanners.len()).collect();

    while !scanner_ids.is_empty() {
        for id in scanner_ids.clone() {
            let is_match = find_matching_scanners(&scanners[0], &scanners[id]);
            if is_match {
                let matching_beacons_pairs =
                    find_matching_beacons_pairs(&scanners[0], &scanners[id]);
                let rotation = determine_rotation(&matching_beacons_pairs);
                let displacement =
                    determine_displacement(&matching_beacons_pairs, rotation, &mut scanners[id]);
                let aligned_beacons =
                    rotate_and_align_beacons(&scanners[id], rotation, displacement);

                scanners[0].beacons.extend(aligned_beacons);
                scanners[0].beacons.sort_unstable();
                scanners[0].beacons.dedup();
                scanners[0].calculate_pairwise_distances();
                scanner_ids.retain(|&scanner_id| scanner_id != id);
                break;
            }
        }
    }

    let mut max_distance_between_scanners = 0;

    for (index, scanner) in scanners.iter().enumerate() {
        for other_scanner in &scanners[index..] {
            let distance_between_scanners = scanner
                .position
                .unwrap()
                .calculate_distance(&other_scanner.position.unwrap())
                .iter()
                .sum();
            if max_distance_between_scanners < distance_between_scanners {
                max_distance_between_scanners = distance_between_scanners
            }
        }
    }

    max_distance_between_scanners
}

pub fn solve() -> String {
    let mut scanners_data = process_data("./input/19.txt");
    let result = find_maximum_distance_between_scanners(&mut scanners_data);
    format!("Day 19: Beacon Scanner (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_19_beacon_scanner::part_2::*;

    #[test]
    fn read_data_from_file_and_convert_to_vector() {
        let mut scanners_data = process_data("./test_input/19.txt");
        assert_eq!(scanners_data.len(), 5);
        assert_eq!(scanners_data[0].beacons.len(), 25);
        scanners_data[1].calculate_pairwise_distances();
        assert_eq!(scanners_data[1].pairwise_distances.len(), 300);
        let result = find_maximum_distance_between_scanners(&mut scanners_data);
        assert_eq!(result, 3621);
    }
}
