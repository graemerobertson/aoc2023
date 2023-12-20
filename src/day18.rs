use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
}

fn in_path(path: &HashSet<Range>, x: isize, y: isize) -> bool {
    for range in path {
        if x >= range.min_x && x <= range.max_x && y >= range.min_y && y <= range.max_y {
            return true;
        }
    }
    false
}

// Slightly random function that returns the path instances in column y, and a the number of
// contiguous columns that are identical to column y, starting at column y and going right.
//
// The path instances are returned as a vector of tuples, where the first element is the start
// of a path instance, and the second element is the end of the path instance.
fn find_path_in_column_y(path: &HashSet<Range>, y: isize) -> (Vec<(isize, isize)>, isize) {
    let mut path_instances: Vec<(isize, isize)> = Vec::new();
    let mut max_y_that_looks_identical = isize::MAX;
    for range in path {
        if (y < range.max_y && y > range.min_y) || (y == range.max_y && y == range.min_y) {
            max_y_that_looks_identical = std::cmp::min(max_y_that_looks_identical, range.max_y);
            path_instances.push((range.min_x, range.max_x));
        }
    }
    path_instances.sort_by(|a, b| a.0.cmp(&b.0));

    for range in path {
        if range.min_y == range.max_y
            && range.min_y > y
            && range.min_y <= max_y_that_looks_identical
        {
            max_y_that_looks_identical = range.min_y - 1;
        }
    }
    if max_y_that_looks_identical == isize::MAX {
        panic!("No path either in column y, or to the right of column y - what are we doing here?!")
    }
    (path_instances, 1 + max_y_that_looks_identical - y)
}

fn lagoon_interior_volume(path: &HashSet<Range>) -> usize {
    let max_y = path.iter().map(|r| r.max_y).max().unwrap();
    let min_y = path.iter().map(|r| r.min_y).min().unwrap();
    let mut count = 0;
    let mut it = min_y..=max_y;
    while let Some(y) = it.next() {
        let mut current_x_index: isize = 0;
        let mut inside: bool = false;
        let (path_instances, number_of_identical_cols) = find_path_in_column_y(path, y);
        for path_instance in path_instances {
            if inside {
                count += ((path_instance.0 - current_x_index) * number_of_identical_cols) as usize;
            }
            if path_instance.0 == path_instance.1
                || (in_path(path, path_instance.0, y + 1) && in_path(path, path_instance.1, y - 1))
                || (in_path(path, path_instance.0, y - 1) && in_path(path, path_instance.1, y + 1))
            {
                inside = !inside;
            }
            current_x_index = path_instance.1 + 1;
        }
        if number_of_identical_cols > 1 {
            it.nth(number_of_identical_cols as usize - 2);
        }
    }
    count
}

pub(crate) fn day18() {
    let f: File = File::open("data/day18.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut part1_path: HashSet<Range> = HashSet::new();
    let mut part1_path_len: usize = 0;
    let mut current_location: (isize, isize) = (0, 0);
    for line in &lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split[0].parse::<char>().unwrap();
        let distance = split[1].parse::<usize>().unwrap();
        let new_location: (isize, isize) = match direction {
            'U' => (current_location.0 - distance as isize, current_location.1),
            'D' => (current_location.0 + distance as isize, current_location.1),
            'R' => (current_location.0, current_location.1 + distance as isize),
            'L' => (current_location.0, current_location.1 - distance as isize),
            _ => panic!("Unknown direction"),
        };
        part1_path.insert(Range {
            min_x: std::cmp::min(current_location.0, new_location.0),
            max_x: std::cmp::max(current_location.0, new_location.0),
            min_y: std::cmp::min(current_location.1, new_location.1),
            max_y: std::cmp::max(current_location.1, new_location.1),
        });
        part1_path_len += distance;
        current_location = new_location;
    }

    println!(
        "Day 18 Part 1: {}",
        lagoon_interior_volume(&part1_path) + part1_path_len
    );

    let mut part2_path: HashSet<Range> = HashSet::new();
    let mut part2_path_len: usize = 0;
    let mut current_location: (isize, isize) = (0, 0);
    for line in &lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let temp = split[2]
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        let direction = temp.chars().last().unwrap();
        let distance = usize::from_str_radix(&temp[0..temp.len() - 1], 16).unwrap();
        let new_location: (isize, isize) = match direction {
            '3' => (current_location.0 - distance as isize, current_location.1),
            '1' => (current_location.0 + distance as isize, current_location.1),
            '0' => (current_location.0, current_location.1 + distance as isize),
            '2' => (current_location.0, current_location.1 - distance as isize),
            _ => panic!("Unknown direction"),
        };
        part2_path.insert(Range {
            min_x: std::cmp::min(current_location.0, new_location.0),
            max_x: std::cmp::max(current_location.0, new_location.0),
            min_y: std::cmp::min(current_location.1, new_location.1),
            max_y: std::cmp::max(current_location.1, new_location.1),
        });
        part2_path_len += distance;
        current_location = new_location;
    }

    println!(
        "Day 18 Part 2: {}",
        lagoon_interior_volume(&part2_path) + part2_path_len
    );
}
