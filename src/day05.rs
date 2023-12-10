use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value < self.end
    }
}

pub struct Mapping {
    source_range: Range,
    dest_range: Range,
}

impl Mapping {
    fn map(&self, value: u64) -> u64 {
        if self.source_range.contains(value) {
            value + self.dest_range.start - self.source_range.start
        } else {
            value
        }
    }

    fn map_dest_to_source(&self, value: u64) -> u64 {
        if self.dest_range.contains(value) {
            value + self.source_range.start - self.dest_range.start
        } else {
            value
        }
    }
}

pub(crate) fn day05() {
    let f: File = File::open("data/day05.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let part1_seeds: Vec<u64> = lines[0].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let part2_seeds: Vec<Range> = part1_seeds
        .chunks(2)
        .map(|x| Range {
            start: x[0],
            end: x[0] + x[1],
        })
        .collect();

    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let mut current_mapping: Option<Vec<Mapping>> = None;
    for line in lines {
        if line.ends_with(':') {
            current_mapping = Some(vec![]);
            continue;
        } else if line.is_empty() && current_mapping.is_some() {
            mappings.push(current_mapping.unwrap());
            current_mapping = None;
            continue;
        }

        if let Some(ref mut current_mapping_vec) = current_mapping {
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            current_mapping_vec.push(Mapping {
                source_range: Range {
                    start: numbers[1],
                    end: numbers[1] + numbers[2],
                },
                dest_range: Range {
                    start: numbers[0],
                    end: numbers[0] + numbers[2],
                },
            });
        }
    }
    if let Some(current_mapping_vec) = current_mapping {
        mappings.push(current_mapping_vec);
    }

    let mut part1_locations: Vec<u64> = vec![];
    for seed in part1_seeds {
        let mut final_location = seed;
        for mapping in &mappings {
            for inner_mapping in mapping {
                let mapped_location = inner_mapping.map(final_location);
                if final_location != mapped_location {
                    final_location = mapped_location;
                    break;
                }
            }
        }
        part1_locations.push(final_location);
    }
    println!("Day 5 part 1: {}", part1_locations.iter().min().unwrap());

    // This assumes there's a 1:1 mapping between seeds and locations, which seems heavily implied
    // from the question.
    //
    // The plan here is to start from 0 and find the first location that maps back to a seed. In
    // order to make this more efficient, we try to look at a range of numbers on each loop. We
    // always shorten the length of this range so that it falls within a single mapping.
    //
    // Yeh, this code is very hard to reason about and just generally a bit shit.
    let mut length_of_range: u64 = 0;
    for potential_location in 0..4294967295 {
        if length_of_range > 1 {
            length_of_range -= 1;
            continue;
        }
        length_of_range = 4294967295;
        let mut initial_seed: u64 = potential_location;
        for mapping in mappings.iter().rev() {
            let mut hit_mapping = false;
            let mut might_need_to_reduce_current_range = false;
            let mut closest_mapping_distance: u64 = 4294967295;
            for inner_mapping in mapping {
                let mapped_seed = inner_mapping.map_dest_to_source(initial_seed);
                if initial_seed != mapped_seed {
                    initial_seed = mapped_seed;
                    length_of_range =
                        std::cmp::min(length_of_range, inner_mapping.dest_range.end - initial_seed);
                    hit_mapping = true;
                    break;
                }
                if initial_seed < inner_mapping.dest_range.start {
                    might_need_to_reduce_current_range = true;
                    closest_mapping_distance = std::cmp::min(
                        closest_mapping_distance,
                        inner_mapping.dest_range.start - initial_seed,
                    );
                }
            }
            if !hit_mapping && might_need_to_reduce_current_range {
                length_of_range = std::cmp::min(length_of_range, closest_mapping_distance);
            }
        }
        for seed_range in &part2_seeds {
            // Lazy optimization.
            if (initial_seed < seed_range.start
                && initial_seed + length_of_range < seed_range.start)
                || (initial_seed > seed_range.end)
            {
                continue;
            }

            for offset in 0..length_of_range {
                if initial_seed + offset >= seed_range.start
                    && initial_seed + offset < seed_range.end
                {
                    println!("Day 5 part 2: {}", potential_location + offset);
                    return;
                }
            }
        }
    }
}
