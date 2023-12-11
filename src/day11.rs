use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Point {
    Galaxy,
    Empty,
}

fn expand_universe(base_universe: &Vec<Vec<Point>>, expansion_factor: usize) -> Vec<Vec<Point>> {
    let len = base_universe[0].len();
    let mut half_expanded_universe: Vec<Vec<Point>> = vec![];
    for line in base_universe {
        half_expanded_universe.push(line.clone());
        if line.iter().all(|x| matches!(x, Point::Empty)) {
            for _ in 0..expansion_factor {
                half_expanded_universe.push(line.clone());
            }
        }
    }

    let mut iters: Vec<_> = half_expanded_universe
        .into_iter()
        .map(|n| n.into_iter())
        .collect();
    half_expanded_universe = (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<Point>>()
        })
        .collect();

    let mut expanded_universe: Vec<Vec<Point>> = vec![];
    for line in half_expanded_universe {
        expanded_universe.push(line.clone());
        if line.iter().all(|x| matches!(x, Point::Empty)) {
            for _ in 0..expansion_factor {
                expanded_universe.push(line.clone());
            }
        }
    }

    expanded_universe
}

fn find_galaxies(universe: &[Vec<Point>]) -> HashSet<(usize, usize)> {
    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if matches!(col, Point::Galaxy) {
                galaxies.insert((i, j));
            }
        }
    }
    galaxies
}

fn count_distances(galaxies: &HashSet<(usize, usize)>) -> isize {
    let mut processed_galaxies: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    for galaxy in galaxies {
        for galaxy_to_compare in galaxies {
            if processed_galaxies.contains(galaxy_to_compare) {
                continue;
            } else {
                sum += (galaxy.0 as isize - galaxy_to_compare.0 as isize).abs()
                    + (galaxy.1 as isize - galaxy_to_compare.1 as isize).abs();
            }
        }
        processed_galaxies.insert(*galaxy);
    }
    sum
}

pub(crate) fn day11() {
    let f: File = File::open("data/day11.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut base_universe: Vec<Vec<Point>> = vec![];
    for line in lines {
        let mut row: Vec<Point> = vec![];
        for c in line.chars() {
            match c {
                '.' => row.push(Point::Empty),
                '#' => row.push(Point::Galaxy),
                _ => panic!("Unknown character"),
            }
        }
        base_universe.push(row.clone());
    }

    let once_expanded_universe: Vec<Vec<Point>> = expand_universe(&base_universe, 1);
    let once_expanded_galaxies: HashSet<(usize, usize)> = find_galaxies(&once_expanded_universe);
    let once_expanded_distances: isize = count_distances(&once_expanded_galaxies);
    println!("Day 11 part 1: {}", once_expanded_distances);
    let twice_expanded_universe: Vec<Vec<Point>> = expand_universe(&base_universe, 2);
    let twice_expanded_galaxies: HashSet<(usize, usize)> = find_galaxies(&twice_expanded_universe);
    let twice_expanded_distances: isize = count_distances(&twice_expanded_galaxies);
    println!(
        "Day 11 part 2: {}",
        ((twice_expanded_distances - once_expanded_distances) * 999998)
    );
}
