use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum TileType {
    Path,
    IcySlope((isize, isize)),
}

// Map of junctions to their adjacent junctions plus distance.
type Nodes = HashMap<Junction, Vec<(Junction, usize)>>;
type Junction = (isize, isize);

// This function builds up the Nodes map by recursively traversing the path, but it's a bit of a
// mess and I can't be bothered to tidy it up.
fn build_junction_graph(
    start: (isize, isize),
    first_step: (isize, isize),
    path: &HashMap<(isize, isize), TileType>,
    junction_graph: &mut Nodes,
) {
    let mut path_to_next_junction: HashSet<(isize, isize)> = HashSet::new();
    path_to_next_junction.insert(start);
    path_to_next_junction.insert(first_step);
    let mut next_step = first_step;
    let mut distance = 0;
    loop {
        distance += 1;
        let mut neighbours: Vec<(isize, isize)> = Vec::new();
        for delta in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            if path.contains_key(&(next_step.0 + delta.0, next_step.1 + delta.1))
                && !path_to_next_junction.contains(&(next_step.0 + delta.0, next_step.1 + delta.1))
            {
                neighbours.push((next_step.0 + delta.0, next_step.1 + delta.1));
            }
        }
        if neighbours.is_empty() {
            let x = if next_step.0 == 1 { 0 } else { next_step.0 + 1 };
            if let std::collections::hash_map::Entry::Vacant(e) = junction_graph.entry(start) {
                e.insert(vec![((x, next_step.1), distance + 1)]);
            } else {
                junction_graph
                    .get_mut(&start)
                    .unwrap()
                    .push(((x, next_step.1), distance + 1));
            }
            break;
        }
        if neighbours.len() == 1 {
            path_to_next_junction.insert(neighbours[0]);
            next_step = neighbours[0];
        } else {
            if let std::collections::hash_map::Entry::Vacant(e) = junction_graph.entry(start) {
                e.insert(vec![(next_step, distance)]);
            } else {
                junction_graph
                    .get_mut(&start)
                    .unwrap()
                    .push((next_step, distance));
            }
            if !junction_graph.contains_key(&next_step) {
                for delta in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    if path.contains_key(&(next_step.0 + delta.0, next_step.1 + delta.1)) {
                        build_junction_graph(
                            next_step,
                            (next_step.0 + delta.0, next_step.1 + delta.1),
                            path,
                            junction_graph,
                        );
                    }
                }
            }
            break;
        }
    }
}

fn count_max_steps_direction(
    start: (isize, isize),
    end: (isize, isize),
    next_step: (isize, isize),
    path: &HashMap<(isize, isize), TileType>,
) -> isize {
    let next = (start.0 + next_step.0, start.1 + next_step.1);
    if next == end {
        return 1;
    }
    let mut count: isize = 0;
    if path.contains_key(&next) {
        match path.get(&next).unwrap() {
            TileType::Path => {
                count += 1;
                let mut remaining_path = path.clone();
                remaining_path.remove(&next);
                count += count_max_steps(next, end, &remaining_path);
            }
            TileType::IcySlope(delta) => {
                if path.contains_key(&(next.0 + delta.0, next.1 + delta.1)) {
                    count += 2;
                    let mut remaining_path = path.clone();
                    remaining_path.remove(&next);
                    remaining_path.remove(&(next.0 + delta.0, next.1 + delta.1));
                    count +=
                        count_max_steps((next.0 + delta.0, next.1 + delta.1), end, &remaining_path);
                }
            }
        }
    }
    count
}

fn count_max_steps(
    start: (isize, isize),
    end: (isize, isize),
    path: &HashMap<(isize, isize), TileType>,
) -> isize {
    let mut counts: HashSet<isize> = HashSet::new();
    counts.insert(count_max_steps_direction(start, end, (-1, 0), path));
    counts.insert(count_max_steps_direction(start, end, (1, 0), path));
    counts.insert(count_max_steps_direction(start, end, (0, 1), path));
    counts.insert(count_max_steps_direction(start, end, (0, -1), path));
    *counts.iter().max().unwrap()
}

fn count_max_steps_part2(
    start: (isize, isize),
    end: (isize, isize),
    path: &Nodes,
) -> Option<isize> {
    let mut counts: HashSet<isize> = HashSet::new();
    let next_junctions = path.get(&start).unwrap();
    let mut remaining_path = path.clone();
    remaining_path.remove(&start);
    for j in next_junctions {
        if j.0 == end {
            counts.insert(j.1 as isize);
        } else if remaining_path.contains_key(&j.0) {
            if let Some(count) = count_max_steps_part2(j.0, end, &remaining_path) {
                counts.insert(j.1 as isize + count);
            }
        }
    }
    counts.iter().max().copied()
}

pub(crate) fn day23() {
    let f: File = File::open("data/day23.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut path: HashMap<(isize, isize), TileType> = HashMap::new();
    let height = lines.len();
    let width = lines[0].len();
    let mut start: (isize, isize) = (0, 0);
    let mut end: (isize, isize) = (0, 0);
    (0..height).for_each(|r| {
        let mut line = lines[r].chars();
        let row = r as isize;
        (0..width).for_each(|c| {
            let col = c as isize;
            match line.next().unwrap() {
                '.' => {
                    if row == 0 {
                        start = (row, col);
                    } else if row == height as isize - 1 {
                        end = (row, col);
                    } else {
                        path.insert((row, col), TileType::Path);
                    }
                }
                '^' => {
                    path.insert((row, col), TileType::IcySlope((-1, 0)));
                }
                'v' => {
                    path.insert((row, col), TileType::IcySlope((1, 0)));
                }
                '<' => {
                    path.insert((row, col), TileType::IcySlope((0, -1)));
                }
                '>' => {
                    path.insert((row, col), TileType::IcySlope((0, 1)));
                }
                '#' => (),
                _ => panic!("Invalid character in input"),
            }
        })
    });

    // Brute force part 1.
    println!("Day 23 Part 1: {:?}", count_max_steps(start, end, &path));

    // Brute forcing part 2 was less successful. Simplify the problem by reducing the path to a
    // graph of junctions, and brute force those. Still slow, but good enough for me.
    let mut junctions: Nodes = HashMap::new();
    build_junction_graph(start, (start.0 + 1, start.1), &path, &mut junctions);
    println!(
        "Day 23 Part 2: {:?}",
        count_max_steps_part2(start, end, &junctions).unwrap()
    );
}
