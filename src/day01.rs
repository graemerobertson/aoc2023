use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day01() {
    let f: File = File::open("data/day01.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let calibration_lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let part_1_valid_digits: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let part_2_valid_digits: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    println!(
        "Day 1 part 1: {}",
        solve(calibration_lines.clone(), &part_1_valid_digits)
    );
    println!(
        "Day 2 part 2: {}",
        solve(calibration_lines, &part_2_valid_digits)
    );
}

fn solve(calibration_lines: Vec<String>, valid_digits: &HashMap<&str, u32>) -> u32 {
    let mut sum = 0;
    for line in calibration_lines {
        let mut numbers = Vec::new();
        // Walk over the line sniffing out valid digits, and adding them to the numbers vec
        for mut index in 0..line.len() {
            for digit in valid_digits.keys() {
                if line.len() >= index + digit.len() && &&line[index..index + digit.len()] == digit
                {
                    numbers.push(valid_digits.get(digit).unwrap());
                    index += digit.len();
                }
            }
        }
        sum += format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap()
    }
    sum
}
