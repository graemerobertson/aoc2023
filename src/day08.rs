use num_integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day08() {
    let f: File = File::open("data/day08.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let instructions: Vec<char> = lines[0].chars().collect();
    let mut mappings: HashMap<String, (String, String)> = HashMap::new();
    let mut part2_starting_points: Vec<String> = Vec::new();
    for line in &lines[2..] {
        let split: Vec<&str> = line.split(" = ").collect();
        let mut rhs = split[1].replace('(', "");
        rhs = rhs.replace(')', "");
        let inner_split: Vec<&str> = rhs.split(", ").collect();
        mappings.insert(
            split[0].to_string(),
            (inner_split[0].to_string(), inner_split[1].to_string()),
        );
        if split[0].ends_with('A') {
            part2_starting_points.push(split[0].to_string());
        }
    }

    let mut part1_current_location = "AAA".to_string();
    let mut part1_step_count = 0;
    'outer: loop {
        for instruction in &instructions {
            part1_step_count += 1;
            if instruction == &'L' {
                part1_current_location = mappings[&part1_current_location].0.to_string();
            } else {
                part1_current_location = mappings[&part1_current_location].1.to_string();
            }
            if part1_current_location == "ZZZ" {
                println!("Day 8 part 1: {}", part1_step_count);
                break 'outer;
            }
        }
    }

    // Calculate each path independently and take the least common multiple of the step counts.
    let mut part2_step_counts: Vec<u64> = vec![];
    for location in part2_starting_points {
        let mut step_count = 0;
        let mut new_location = location;
        'part2_outer: loop {
            for instruction in &instructions {
                step_count += 1;
                if instruction == &'L' {
                    new_location = mappings[&new_location].0.to_string();
                } else {
                    new_location = mappings[&new_location].1.to_string();
                }

                if new_location.ends_with('Z') {
                    part2_step_counts.push(step_count);
                    break 'part2_outer;
                }
            }
        }
    }
    println!(
        "Day 8 part 2: {}",
        part2_step_counts.iter().fold(1, |a, b| lcm(a, *b))
    );
}
