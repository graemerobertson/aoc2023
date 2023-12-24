use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum TileType {
    Path,
    IcySlope((isize, isize)),
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

fn count_max_steps_direction_part2(
    start: (isize, isize),
    end: (isize, isize),
    next_step: (isize, isize),
    path: &HashMap<(isize, isize), TileType>,
    current_count: &mut usize,
    best: &mut usize,
) -> Option<isize> {
    let next = (start.0 + next_step.0, start.1 + next_step.1);
    if next == end {
        *current_count += 1;
        if current_count > best {
            *best = *current_count;
        }
        return Some(1);
    }
    if path.contains_key(&next) {
        *current_count += 1;
        let mut remaining_path = path.clone();
        remaining_path.remove(&next);
        if let Some(count) = count_max_steps_part2(next, end, &remaining_path, current_count, best)
        {
            return Some(count + 1);
        }
    }
    None
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
    path: &HashMap<(isize, isize), TileType>,
    current_count: &mut usize,
    best: &mut usize,
) -> Option<isize> {
    let mut counts: HashSet<isize> = HashSet::new();
    if let Some(count) =
        count_max_steps_direction_part2(start, end, (-1, 0), path, &mut current_count.clone(), best)
    {
        counts.insert(count);
    }
    if let Some(count) =
        count_max_steps_direction_part2(start, end, (1, 0), path, &mut current_count.clone(), best)
    {
        counts.insert(count);
    }
    if let Some(count) =
        count_max_steps_direction_part2(start, end, (0, -1), path, &mut current_count.clone(), best)
    {
        counts.insert(count);
    }
    if let Some(count) =
        count_max_steps_direction_part2(start, end, (0, 1), path, &mut current_count.clone(), best)
    {
        counts.insert(count);
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
    println!("Day 23 Part 1: {:?}", count_max_steps(start, end, &path));
    let mut current_count: usize = 0;
    let mut best: usize = 6250;
    println!(
        "Day 23 Part 2: {:?}",
        count_max_steps_part2(start, end, &path, &mut current_count, &mut best).unwrap()
    );
}
