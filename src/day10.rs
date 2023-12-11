use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) fn day10() {
    let f: File = File::open("data/day10.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut grid: HashMap<Coord, char> = HashMap::new();
    let mut start: Coord = Coord { row: 0, col: 0 };
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let height = lines.len();
    let width = lines[0].len();

    // Build grid.
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid.insert(Coord { row, col }, c);
            if c == 'S' {
                start = Coord { row, col };
            }
        }
    }

    // Figure out the initial set of points a path could start from
    let mut plausible_start_neighbours: HashMap<Coord, Direction> = HashMap::new();
    let up_connector: Option<Coord> = get_up_connector(&start, &grid);
    if let Some(up_connector) = up_connector {
        plausible_start_neighbours.insert(up_connector, Direction::Down);
    }
    let down_connector: Option<Coord> = get_down_connector(&start, &grid);
    if let Some(down_connector) = down_connector {
        plausible_start_neighbours.insert(down_connector, Direction::Up);
    }
    let left_connector: Option<Coord> = get_left_connector(&start, &grid);
    if let Some(left_connector) = left_connector {
        plausible_start_neighbours.insert(left_connector, Direction::Right);
    }
    let right_connector: Option<Coord> = get_right_connector(&start, &grid);
    if let Some(right_connector) = right_connector {
        plausible_start_neighbours.insert(right_connector, Direction::Left);
    }

    // For each of the plausible start points, try to find a path that connects back to the start.
    'outer: for start_neighbour in plausible_start_neighbours.keys() {
        let mut path: HashSet<Coord> = HashSet::new();
        path.insert(start.clone());
        path.insert(start_neighbour.clone());

        let mut next_point_in_path: Coord = start_neighbour.clone();
        // We track which direction we came from, so we don't go back the way we came.
        let mut next_prev_direction = plausible_start_neighbours
            .get(start_neighbour)
            .unwrap()
            .clone();
        loop {
            let (connector, prev_direction): (Option<Coord>, Direction) =
                get_connecting_point(&next_point_in_path, &grid, &next_prev_direction);
            if let Some(connector) = connector {
                next_point_in_path = connector;
                next_prev_direction = prev_direction.clone();

                if path.contains(&next_point_in_path) {
                    panic!("Hit incomplete loop - this shouldn't be possible");
                }
                path.insert(next_point_in_path.clone());

                if plausible_start_neighbours.contains_key(&next_point_in_path) {
                    // We've found a path that connects back to the start.
                    println!("Day 10 part 1: {}", path.len() / 2);

                    // Unbelievably tedious code to figure out what type of pipe S is.
                    let mut directions_set: HashSet<Direction> = HashSet::new();
                    directions_set.insert(
                        plausible_start_neighbours
                            .get(&next_point_in_path)
                            .unwrap()
                            .clone(),
                    );
                    directions_set.insert(
                        plausible_start_neighbours
                            .get(start_neighbour)
                            .unwrap()
                            .clone(),
                    );
                    if directions_set.contains(&Direction::Up)
                        && directions_set.contains(&Direction::Down)
                    {
                        grid.insert(start.clone(), '|');
                    } else if directions_set.contains(&Direction::Left)
                        && directions_set.contains(&Direction::Right)
                    {
                        grid.insert(start.clone(), '-');
                    } else if directions_set.contains(&Direction::Down)
                        && directions_set.contains(&Direction::Left)
                    {
                        grid.insert(start.clone(), 'L');
                    } else if directions_set.contains(&Direction::Down)
                        && directions_set.contains(&Direction::Right)
                    {
                        grid.insert(start.clone(), 'J');
                    } else if directions_set.contains(&Direction::Up)
                        && directions_set.contains(&Direction::Left)
                    {
                        grid.insert(start.clone(), 'F');
                    } else if directions_set.contains(&Direction::Up)
                        && directions_set.contains(&Direction::Right)
                    {
                        grid.insert(start.clone(), '7');
                    }

                    // For each point in the grid, count the number of times we have to cross the
                    // path to get to the boundary.
                    let mut number_of_points_inside_path = 0;
                    for row in 0..height {
                        for col in 0..width {
                            let mut number_of_boundaries_crossed = 0;
                            if !path.contains(&Coord { row, col }) {
                                // Erm... Best not to dwell on this code too much. We need to
                                // track bends in a way that is both hard for me to think about and
                                // also apparently hard for me to code.
                                let mut found_f: usize = 0;
                                let mut found_l: usize = 0;
                                let mut found_7: usize = 0;
                                let mut found_j: usize = 0;
                                for new_row in 0..row {
                                    if path.contains(&Coord { row: new_row, col }) {
                                        let point = grid.get(&Coord { row: new_row, col }).unwrap();
                                        if point == &'-' {
                                            number_of_boundaries_crossed += 1;
                                        } else if point == &'F' {
                                            if found_j > 0 {
                                                number_of_boundaries_crossed += 1;
                                                found_j -= 1;
                                            } else {
                                                found_f += 1;
                                            }
                                        } else if point == &'L' {
                                            if found_7 > 0 {
                                                number_of_boundaries_crossed += 1;
                                                found_7 -= 1;
                                            } else {
                                                found_l += 1;
                                            }
                                        } else if point == &'7' {
                                            if found_l > 0 {
                                                number_of_boundaries_crossed += 1;
                                                found_l -= 1;
                                            } else {
                                                found_7 += 1;
                                            }
                                        } else if point == &'J' {
                                            if found_f > 0 {
                                                number_of_boundaries_crossed += 1;
                                                found_f -= 1;
                                            } else {
                                                found_j += 1;
                                            }
                                        }
                                    }
                                }
                                if number_of_boundaries_crossed % 2 == 1 {
                                    number_of_points_inside_path += 1;
                                }
                            }
                        }
                    }
                    println!("Day 10 part 2: {}", number_of_points_inside_path);
                    return;
                }
            } else {
                // We've hit a dead end, move on to the next potential path.
                continue 'outer;
            }
        }
    }
}

// Search for a point in `grid` that directly connects to `coord`, excluding in the direction
// indicated by the `exclude_direction`.
//
// Returns the connecting point and the direction of `coord` *_from_* that connecting point.
pub fn get_connecting_point(
    coord: &Coord,
    grid: &HashMap<Coord, char>,
    exclude_direction: &Direction,
) -> (Option<Coord>, Direction) {
    let current_coord = grid.get(coord).unwrap_or(&'.');
    if !matches!(exclude_direction, Direction::Up)
        && (current_coord == &'|' || current_coord == &'J' || current_coord == &'L')
    {
        return (get_up_connector(coord, grid), Direction::Down);
    }
    if !matches!(exclude_direction, Direction::Down)
        && (current_coord == &'|' || current_coord == &'7' || current_coord == &'F')
    {
        return (get_down_connector(coord, grid), Direction::Up);
    }
    if !matches!(exclude_direction, Direction::Left)
        && (current_coord == &'-' || current_coord == &'7' || current_coord == &'J')
    {
        return (get_left_connector(coord, grid), Direction::Right);
    }
    if !matches!(exclude_direction, Direction::Right)
        && (current_coord == &'-' || current_coord == &'F' || current_coord == &'L')
    {
        return (get_right_connector(coord, grid), Direction::Left);
    }
    (None, Direction::Up)
}

pub fn get_up_connector(coord: &Coord, grid: &HashMap<Coord, char>) -> Option<Coord> {
    let up_coord = Coord {
        row: coord.row - 1,
        col: coord.col,
    };
    if check_point_connects(&up_coord, grid, &['|', '7', 'F']) {
        return Some(up_coord);
    }
    None
}

pub fn get_down_connector(coord: &Coord, grid: &HashMap<Coord, char>) -> Option<Coord> {
    let down_coord = Coord {
        row: coord.row + 1,
        col: coord.col,
    };
    if check_point_connects(&down_coord, grid, &['|', 'J', 'L']) {
        return Some(down_coord);
    }
    None
}

pub fn get_left_connector(coord: &Coord, grid: &HashMap<Coord, char>) -> Option<Coord> {
    let left_coord = Coord {
        row: coord.row,
        col: coord.col - 1,
    };
    if check_point_connects(&left_coord, grid, &['-', 'F', 'L']) {
        return Some(left_coord);
    }
    None
}

pub fn get_right_connector(coord: &Coord, grid: &HashMap<Coord, char>) -> Option<Coord> {
    let right_coord = Coord {
        row: coord.row,
        col: coord.col + 1,
    };
    if check_point_connects(&right_coord, grid, &['-', '7', 'J']) {
        return Some(right_coord);
    }
    None
}

pub fn check_point_connects(
    coord: &Coord,
    grid: &HashMap<Coord, char>,
    match_list: &[char],
) -> bool {
    if match_list.contains(grid.get(coord).unwrap_or(&'.')) {
        return true;
    }
    false
}
