use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const NUMBER_OF_STEPS: usize = 64;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Point {
    Rock,
    Garden,
}

pub(crate) fn day21() {
    let f: File = File::open("data/day21.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut grid: Vec<Vec<Point>> = vec![];
    let mut start: ((usize, usize), (isize, isize)) = ((0, 0), (0, 0));
    for (row_index, line) in lines.iter().enumerate() {
        let mut row: Vec<Point> = vec![];
        for (col_index, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(Point::Rock),
                '.' => {
                    row.push(Point::Garden);
                }
                'S' => {
                    row.push(Point::Garden);
                    start = ((row_index, col_index), (0, 0));
                }
                _ => panic!("Unknown character"),
            }
        }
        grid.push(row);
    }
    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;

    let mut attainable_plots: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    attainable_plots.insert(start);
    for _ in 1..=NUMBER_OF_STEPS {
        let mut next_attainable_plots: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
        for (grid_coords, plot_coords) in attainable_plots {
            let (grid_row, grid_col) = grid_coords;
            let (plot_row, plot_col) = plot_coords;
            if grid_row > 0 {
                if grid[grid_row - 1][grid_col] == Point::Garden {
                    next_attainable_plots.insert(((grid_row - 1, grid_col), plot_coords));
                }
            } else if grid[max_row][grid_col] == Point::Garden {
                next_attainable_plots.insert(((max_row, grid_col), (plot_row - 1, plot_col)));
            }
            if grid_row < max_row {
                if grid[grid_row + 1][grid_col] == Point::Garden {
                    next_attainable_plots.insert(((grid_row + 1, grid_col), plot_coords));
                }
            } else if grid[0][grid_col] == Point::Garden {
                next_attainable_plots.insert(((0, grid_col), (plot_row + 1, plot_col)));
            }
            if grid_col > 0 {
                if grid[grid_row][grid_col - 1] == Point::Garden {
                    next_attainable_plots.insert(((grid_row, grid_col - 1), plot_coords));
                }
            } else if grid[grid_row][max_col] == Point::Garden {
                next_attainable_plots.insert(((grid_row, max_col), (plot_row, plot_col - 1)));
            }
            if grid_col < max_col {
                if grid[grid_row][grid_col + 1] == Point::Garden {
                    next_attainable_plots.insert(((grid_row, grid_col + 1), plot_coords));
                }
            } else if grid[grid_row][0] == Point::Garden {
                next_attainable_plots.insert(((grid_row, 0), (plot_row, plot_col + 1)));
            }
        }
        attainable_plots = next_attainable_plots;
    }

    println!("Day 21 part 1: {}", attainable_plots.len());
    // I do not understand part 2 at all.
}
