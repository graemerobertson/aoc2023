use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub enum Point {
    Empty,
    ForwardMirror,
    BackMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub struct BeamOfLight {
    position: (usize, usize),
    direction: Direction,
}

// Given a beam of light `beam`, determine the co-ordinates of it's next location, if it exists.
fn find_next_coord(beam: &BeamOfLight, grid: &Vec<Vec<Point>>) -> Option<(usize, usize)> {
    match beam.direction {
        Direction::Up => {
            if beam.position.0 > 0 {
                return Some((beam.position.0 - 1, beam.position.1));
            }
        }
        Direction::Down => {
            if beam.position.0 < grid.len() - 1 {
                return Some((beam.position.0 + 1, beam.position.1));
            }
        }
        Direction::Left => {
            if beam.position.1 > 0 {
                return Some((beam.position.0, beam.position.1 - 1));
            }
        }
        Direction::Right => {
            if beam.position.1 < grid[0].len() - 1 {
                return Some((beam.position.0, beam.position.1 + 1));
            }
        }
    }
    None
}

// Insert this new beam of light into the cache. If it's not already in the cache, also insert it
// into the set of unprocessed light beams.
fn insert_beam(
    beam: BeamOfLight,
    unprocessed_light_beams: &mut HashSet<BeamOfLight>,
    cache: &mut HashSet<BeamOfLight>,
) {
    if cache.insert(beam) {
        unprocessed_light_beams.insert(beam);
    }
}

// A beam of light enters the co-ordinate at `location` from direction `entry_direction`.
//
// Return a vector of beams of light that come out the other side.
fn process_light_beam(
    location: (usize, usize),
    entry_direction: Direction,
    grid: &[Vec<Point>],
) -> Vec<BeamOfLight> {
    match grid[location.0][location.1] {
        Point::Empty => {
            vec![BeamOfLight {
                position: location,
                direction: entry_direction,
            }]
        }
        Point::ForwardMirror => {
            let exit_direction = match entry_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            vec![BeamOfLight {
                position: location,
                direction: exit_direction,
            }]
        }
        Point::BackMirror => {
            let exit_direction = match entry_direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            vec![BeamOfLight {
                position: location,
                direction: exit_direction,
            }]
        }
        Point::HorizontalSplitter => match entry_direction {
            Direction::Up | Direction::Down => {
                vec![
                    BeamOfLight {
                        position: location,
                        direction: Direction::Left,
                    },
                    BeamOfLight {
                        position: location,
                        direction: Direction::Right,
                    },
                ]
            }
            Direction::Left | Direction::Right => {
                vec![BeamOfLight {
                    position: location,
                    direction: entry_direction,
                }]
            }
        },
        Point::VerticalSplitter => match entry_direction {
            Direction::Left | Direction::Right => {
                vec![
                    BeamOfLight {
                        position: location,
                        direction: Direction::Up,
                    },
                    BeamOfLight {
                        position: location,
                        direction: Direction::Down,
                    },
                ]
            }
            Direction::Up | Direction::Down => {
                vec![BeamOfLight {
                    position: location,
                    direction: entry_direction,
                }]
            }
        },
    }
}

pub(crate) fn day16() {
    let f: File = File::open("data/day16.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut grid: Vec<Vec<Point>> = vec![];
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    for (row, line) in lines.iter().enumerate() {
        grid.push(vec![]);
        for c in line.chars() {
            grid[row].push(match c {
                '.' => Point::Empty,
                '/' => Point::ForwardMirror,
                '\\' => Point::BackMirror,
                '-' => Point::HorizontalSplitter,
                '|' => Point::VerticalSplitter,
                _ => panic!("Unknown character"),
            });
        }
    }

    let mut max_energized_tiles = 0;
    let mut entry_points: Vec<(usize, usize, Direction)> = vec![];
    for i in 0..grid.len() {
        entry_points.push((i, 0, Direction::Right));
        entry_points.push((i, grid[0].len() - 1, Direction::Left));
    }
    for i in 0..grid[0].len() {
        entry_points.push((0, i, Direction::Down));
        entry_points.push((grid.len() - 1, i, Direction::Up));
    }
    for entry in entry_points {
        let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
        energized_tiles.insert((entry.0, entry.1));
        let mut beams_of_light: HashSet<BeamOfLight> = HashSet::new();
        let mut cache: HashSet<BeamOfLight> = HashSet::new();
        for new_beam in process_light_beam((entry.0, entry.1), entry.2, &grid) {
            insert_beam(new_beam, &mut beams_of_light, &mut cache);
        }
        loop {
            let beam = *beams_of_light.iter().next().unwrap();
            beams_of_light.remove(&beam);
            let next_coord = find_next_coord(&beam, &grid);
            if let Some(next_coord) = next_coord {
                energized_tiles.insert(next_coord);
                for new_beam in process_light_beam(next_coord, beam.direction, &grid) {
                    insert_beam(new_beam, &mut beams_of_light, &mut cache);
                }
            }
            if beams_of_light.is_empty() {
                break;
            }
        }
        if entry.0 == 0 && entry.1 == 0 && entry.2 == Direction::Right {
            println!("Day 16 part 1: {}", energized_tiles.len());
        }
        if energized_tiles.len() > max_energized_tiles {
            max_energized_tiles = energized_tiles.len();
        }
    }
    println!("Day 16 part 2: {}", max_energized_tiles);
}
