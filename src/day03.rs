use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum SchematicPoint {
    Empty,
    Part(char),
    PartNumber(u32),
}

// We've reached the end of a run of digits, and we want to insert their resolved value into the
// grid at the locations of each of the digits.
//
// Clear `unresolved_digits` when we're finished.
//
// Parameters:
//  - `unresolved_digits` is the collection of digits we've recorded
//  - `row` is the row we've found them on
//  - `col` is the column we found the last digit on
//  - `schematic_grid` is the grid we want to insert into
fn resolve_digits(
    unresolved_digits: &mut Vec<char>,
    row: usize,
    col: usize,
    schematic_grid: &mut HashMap<(usize, usize), SchematicPoint>,
) {
    // Work out what the number is
    let resolved_number = unresolved_digits
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    // Insert it into the grid
    for i in 0..unresolved_digits.len() {
        schematic_grid.insert((row, col - i), SchematicPoint::PartNumber(resolved_number));
    }
    unresolved_digits.clear();
}

pub(crate) fn day03() {
    let f: File = File::open("data/day03.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let schematic_lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let height = schematic_lines.len();
    let width = schematic_lines[0].len();
    let mut schematic_grid: HashMap<(usize, usize), SchematicPoint> = HashMap::new();
    for (row, line) in schematic_lines.iter().enumerate() {
        let mut unresolved_digits: Vec<char> = vec![];
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if !unresolved_digits.is_empty() {
                        resolve_digits(&mut unresolved_digits, row, col - 1, &mut schematic_grid);
                    }
                    schematic_grid.insert((row, col), SchematicPoint::Empty);
                }
                _ => {
                    if c.is_numeric() {
                        unresolved_digits.push(c);
                    } else {
                        if !unresolved_digits.is_empty() {
                            resolve_digits(
                                &mut unresolved_digits,
                                row,
                                col - 1,
                                &mut schematic_grid,
                            );
                        }
                        schematic_grid.insert((row, col), SchematicPoint::Part(c));
                    }
                }
            };
        }
        if !unresolved_digits.is_empty() {
            resolve_digits(&mut unresolved_digits, row, width - 1, &mut schematic_grid);
        }
    }

    let mut part1_part_numbers: Vec<u32> = vec![];
    let mut part2_sum = 0;
    for row in 0..height {
        for col in 0..width {
            if let SchematicPoint::Part(part) = schematic_grid.get(&(row, col)).unwrap() {
                let mut adjacent_part_numbers: HashSet<u32> = HashSet::new();
                for x in 0..3 {
                    for y in 0..3 {
                        if row + y > 0 && col + x > 0 {
                            if let SchematicPoint::PartNumber(n) =
                                schematic_grid.get(&(row + y - 1, col + x - 1)).unwrap()
                            {
                                adjacent_part_numbers.insert(*n);
                            }
                        }
                    }
                }

                if *part == '*' && adjacent_part_numbers.len() == 2 {
                    part2_sum += adjacent_part_numbers.iter().fold(1, |acc, x| acc * x);
                }
                for part in adjacent_part_numbers {
                    part1_part_numbers.push(part);
                }
            }
        }
    }
    println!("Day 3 part 1: {}", part1_part_numbers.iter().sum::<u32>());
    println!("Day 3 part 2: {}", part2_sum);
}
