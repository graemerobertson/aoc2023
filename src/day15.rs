use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Lens {
    label: String,
    focal_length: u32,
}

fn hash(s: &str) -> u32 {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

pub(crate) fn day15() {
    let f: File = File::open("data/day15.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let steps = lines[0].split(',').collect::<Vec<&str>>();
    let mut part1_sum = 0;
    for step in &steps {
        part1_sum += hash(step);
    }
    println!("Day 15 Part 1: {}", part1_sum);

    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    for step in steps {
        if step.contains('=') {
            let split = step.split('=').collect::<Vec<&str>>();
            let lens_label = split[0];
            let box_label = hash(lens_label);
            let focal_length = split[1].parse::<u32>().unwrap();
            let lens = Lens {
                label: lens_label.to_string(),
                focal_length,
            };
            if let Some(lenses) = boxes.get_mut(&box_label) {
                if let Some(p) = lenses.iter().position(|l| l.label == lens_label) {
                    lenses[p] = lens;
                } else {
                    lenses.push(lens);
                }
            } else {
                let lenses = vec![lens];
                boxes.insert(box_label, lenses);
            }
        } else if step.contains('-') {
            let split = step.split('-').collect::<Vec<&str>>();
            let lens_label = split[0];
            let box_label = hash(lens_label);
            if let Some(lenses) = boxes.get_mut(&box_label) {
                if let Some(p) = lenses.iter().position(|l| l.label == lens_label) {
                    lenses.remove(p);
                }
            }
        } else {
            panic!("Invalid step: {}", step);
        }
    }

    let mut part2_sum = 0;
    for (box_label, lenses) in boxes {
        for (index, lens) in lenses.iter().enumerate() {
            part2_sum += (box_label + 1) * (index as u32 + 1) * lens.focal_length;
        }
    }
    println!("Day 15 Part 1: {}", part2_sum);
}
