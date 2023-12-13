use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_col_reflection(pattern: &[String], previous: Option<usize>) -> Option<usize> {
    let mut transposed_pattern: Vec<String> = vec![];
    for i in 0..pattern[0].len() {
        let mut new_row = String::new();
        for c in pattern {
            new_row.push(c.chars().nth(i).unwrap());
        }
        transposed_pattern.push(new_row);
    }
    get_row_reflection(&transposed_pattern, previous)
}

pub fn get_row_reflection(pattern: &[String], previous: Option<usize>) -> Option<usize> {
    for i in 1..pattern.len() {
        // Reflection between row i-1 and row i.
        let mut candidate = true;
        for j in 0..(std::cmp::min(i, pattern.len() - i)) {
            if pattern[i - j - 1] != pattern[i + j] {
                candidate = false;
                break;
            }
        }
        if candidate && Some(i) != previous {
            return Some(i);
        }
    }
    None
}

pub(crate) fn day13() {
    let f: File = File::open("data/day13.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let len = lines.len();
    let mut current_pattern: Vec<String> = vec![];
    for (index, line) in lines.into_iter().enumerate() {
        if !line.is_empty() {
            current_pattern.push(line);
            if index != len - 1 {
                continue;
            }
        }

        let mut orig_row_val = None;
        let mut orig_col_val = None;
        if let Some(val) = get_col_reflection(&current_pattern, None) {
            part1_sum += val;
            orig_col_val = Some(val);
        } else if let Some(val) = get_row_reflection(&current_pattern, None) {
            part1_sum += 100 * val;
            orig_row_val = Some(val);
        } else {
            panic!("No part 1 reflection found");
        }

        let mut part2_found = false;
        'outer: for i in 0..current_pattern.len() {
            for j in 0..current_pattern[0].len() {
                let mut fixed_pattern: Vec<String> = current_pattern.clone();
                if fixed_pattern[i].get(j..j + 1).unwrap() == "." {
                    fixed_pattern[i].replace_range(j..j + 1, "#");
                } else {
                    fixed_pattern[i].replace_range(j..j + 1, ".");
                }
                if let Some(val) = get_col_reflection(&fixed_pattern, orig_col_val) {
                    part2_sum += val;
                    part2_found = true;
                    break 'outer;
                } else if let Some(val) = get_row_reflection(&fixed_pattern, orig_row_val) {
                    part2_sum += 100 * val;
                    part2_found = true;
                    break 'outer;
                }
            }
        }
        if !part2_found {
            panic!("No part 2 reflection found");
        }
        current_pattern.clear();
    }
    println!("Day 13 part 1: {}", part1_sum);
    println!("Day 13 part 2: {}", part2_sum);
}
