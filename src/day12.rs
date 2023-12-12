use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count_possible_arrangements(row: &[char], arrangement: &[u32]) -> u64 {
    let mut count: u64 = 1;
    count
}

pub(crate) fn day12() {
    let f: File = File::open("data/day12.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut sum: u64 = 0;
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let row = split[0].chars().collect::<Vec<char>>();
        let arrangement = split[1]
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        sum += count_possible_arrangements(&row, &arrangement);
    }
    println!("Day 12 part 1: {}", sum);
}
