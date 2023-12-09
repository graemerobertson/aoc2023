use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_next_number(sequence: &Vec<i32>) -> i32 {
    let differences = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();
    if differences.iter().all(|x| *x == 0) {
        *sequence.last().unwrap()
    } else {
        sequence.last().unwrap() + find_next_number(&differences)
    }
}

pub(crate) fn day09() {
    let f: File = File::open("data/day09.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut next_number_sum: i32 = 0;
    let mut previous_number_sum: i32 = 0;
    for line in lines {
        let sequence: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        next_number_sum += find_next_number(&sequence);
        previous_number_sum += find_next_number(&sequence.into_iter().rev().collect());
    }
    println!("Day 9 part 1: {}", next_number_sum);
    println!("Day 9 part 2: {}", previous_number_sum);
}
