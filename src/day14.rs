use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rock {
    RoundRock,
    SquareRock,
}

fn tilt_north(grid: &mut HashMap<(usize, usize), Rock>, height: usize, width: usize) {
    for row in 0..height {
        for col in 0..width {
            if grid.get(&(row, col)) == Some(&Rock::RoundRock) {
                let mut new_row = 0;
                for previous_row in 0..row {
                    if grid.get(&(previous_row, col)).is_some() {
                        new_row = previous_row + 1;
                    }
                }
                grid.remove(&(row, col));
                grid.insert((new_row, col), Rock::RoundRock);
            }
        }
    }
}

fn tilt_west(grid: &mut HashMap<(usize, usize), Rock>, height: usize, width: usize) {
    for row in 0..height {
        for col in 0..width {
            if grid.get(&(row, col)) == Some(&Rock::RoundRock) {
                let mut new_col = 0;
                for previous_col in 0..col {
                    if grid.get(&(row, previous_col)).is_some() {
                        new_col = previous_col + 1;
                    }
                }
                grid.remove(&(row, col));
                grid.insert((row, new_col), Rock::RoundRock);
            }
        }
    }
}

fn tilt_east(grid: &mut HashMap<(usize, usize), Rock>, height: usize, width: usize) {
    for row in 0..height {
        for col in (0..width).rev() {
            if grid.get(&(row, col)) == Some(&Rock::RoundRock) {
                let mut new_col = width - 1;
                for previous_col in (col + 1..width).rev() {
                    if grid.get(&(row, previous_col)).is_some() {
                        new_col = previous_col - 1;
                    }
                }
                grid.remove(&(row, col));
                grid.insert((row, new_col), Rock::RoundRock);
            }
        }
    }
}

fn tilt_south(grid: &mut HashMap<(usize, usize), Rock>, height: usize, width: usize) {
    for row in (0..height).rev() {
        for col in 0..width {
            if grid.get(&(row, col)) == Some(&Rock::RoundRock) {
                let mut new_row = height - 1;
                for previous_row in (row + 1..height).rev() {
                    if grid.get(&(previous_row, col)).is_some() {
                        new_row = previous_row - 1;
                    }
                }
                grid.remove(&(row, col));
                grid.insert((new_row, col), Rock::RoundRock);
            }
        }
    }
}

fn perform_cycle(grid: &mut HashMap<(usize, usize), Rock>, height: usize, width: usize) {
    tilt_north(grid, height, width);
    tilt_west(grid, height, width);
    tilt_south(grid, height, width);
    tilt_east(grid, height, width);
}

fn calculate_total_load_on_the_north_support_beams(
    grid: &HashMap<(usize, usize), Rock>,
    height: usize,
) -> usize {
    grid.iter()
        .filter_map(|(key, val)| {
            if val == &Rock::RoundRock {
                Some(height - key.0)
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn _print_grid(grid: &HashMap<(usize, usize), Rock>, height: usize, width: usize) {
    println!();
    for row in 0..height {
        let mut printable_row = "".to_string();
        for col in 0..width {
            match grid.get(&(row, col)) {
                Some(Rock::RoundRock) => printable_row.push('O'),
                Some(Rock::SquareRock) => printable_row.push('#'),
                None => printable_row.push('.'),
            }
        }
        println!("{}", printable_row);
    }
    println!();
}

pub(crate) fn day14() {
    let f: File = File::open("data/day14.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let height = lines.len();
    let width = lines[0].len();
    let mut grid: HashMap<(usize, usize), Rock> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                'O' => {
                    grid.insert((row, col), Rock::RoundRock);
                }
                '#' => {
                    grid.insert((row, col), Rock::SquareRock);
                }
                _ => panic!("Invalid input"),
            }
        }
    }

    let mut part1_grid = grid.clone();
    tilt_north(&mut part1_grid, height, width);
    println!(
        "Day 14 part 1: {}",
        calculate_total_load_on_the_north_support_beams(&part1_grid, height)
    );

    // Let's find a loop.
    let mut previous_states: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut current_iteration = 0;
    loop {
        current_iteration += 1;
        perform_cycle(&mut grid, height, width);
        let mut keys = grid
            .keys()
            .cloned()
            .collect::<Vec<(usize, usize)>>()
            .clone();
        keys.sort();
        if previous_states.get(&keys).is_some() {
            // We've found a loop! Calculate how big the loop is and fast forward as far as we can
            // without going past 1000000000.
            let previous_iteration = previous_states.get(&keys).unwrap();
            let cycle_length = current_iteration - previous_iteration;
            let skip_to = cycle_length * ((1_000_000_000 - current_iteration) / cycle_length)
                + current_iteration;
            // Now loop until 1000000000 iterations.
            for _ in skip_to..1_000_000_000 {
                perform_cycle(&mut grid, height, width);
            }
            println!(
                "Day 14 part 2: {}",
                calculate_total_load_on_the_north_support_beams(&grid, height)
            );
            break;
        }
        previous_states.insert(keys, current_iteration);
    }
}
