use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day04() {
    let f: File = File::open("data/day04.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let cards = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut part1_sum = 0;
    let mut card_counts: HashMap<usize, u32> = HashMap::new();
    for (index, card) in cards.iter().enumerate() {
        let count_of_current_card: u32 = match card_counts.entry(index) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                e.insert(e.get() + 1);
                *e.get()
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(1);
                1
            }
        };

        let split = card.split(':').collect::<Vec<&str>>()[1]
            .split('|')
            .collect::<Vec<&str>>();
        let winning_numbers = split[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let our_numbers = split[1]
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        part1_sum += i32::pow(
            2,
            (our_numbers.intersection(&winning_numbers).count() - 1) as u32,
        );
        for i in 0..our_numbers.intersection(&winning_numbers).count() {
            match card_counts.entry(index + 1 + i) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    e.insert(e.get() + count_of_current_card);
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(count_of_current_card);
                }
            }
        }
    }

    println!("Day 4 part 1: {}", part1_sum);
    println!("Day 4 part 2: {}", card_counts.values().sum::<u32>());
}
