use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count_possibilities(
    row: &[char],
    arrangement: &[u64],
    known_dot_distributions: &mut HashMap<(Vec<char>, Vec<u64>), usize>,
) -> usize {
    if arrangement.iter().sum::<u64>() as usize + arrangement.len() > row.len() + 1 {
        return 0;
    }
    if let Some(count) = known_dot_distributions.get(&(row.to_vec(), arrangement.to_vec())) {
        return *count;
    }
    let mut count = 0;
    let mut remaining_row = row.to_owned();
    let mut remaining_arrangement = arrangement.to_owned();
    let mut active_run: Option<u64> = None;
    for c in row.iter().rev() {
        remaining_row.pop();
        if c == &'#' {
            match active_run {
                Some(x) => {
                    // We've got an active run of springs
                    if x > 0 {
                        // Great, we were expecting at least one more; continue
                        active_run = Some(x - 1);
                    } else {
                        // The run had just ended - and yet here is another spring. Invalid.
                        known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), 0);
                        return 0;
                    }
                }
                None => {
                    // We've don't have an active run of springs
                    if remaining_arrangement.is_empty() {
                        // We weren't expecting any more springs! Invalid.
                        known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), 0);
                        return 0;
                    }
                    active_run = Some(remaining_arrangement.pop().unwrap() - 1);
                }
            }
        } else if c == &'.' {
            if let Some(x) = active_run {
                // We've got an active run of springs
                if x > 0 {
                    // And we needed at least one more. Invalid.
                    known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), 0);
                    return 0;
                } else {
                    // And that run has now ended.
                    active_run = None;
                }
            } else if remaining_arrangement.iter().sum::<u64>() as usize
                + remaining_arrangement.len()
                > remaining_row.len() + 1
            {
                // We've used another dot and now we've run out of space for everything else. Invalid.
                known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), 0);
                return 0;
            }
        } else if c == &'?' {
            // Now we want to figure out with this question mark can be, and then recurse, calling
            // this function again with the remaining row and arrangement.

            // These will come in handy.
            let mut remaining_row_with_empty = remaining_row.clone();
            remaining_row_with_empty.push('.');
            let mut remaining_row_with_spring = remaining_row.clone();
            remaining_row_with_spring.push('#');

            match active_run {
                Some(x) => {
                    // We're on an active run of springs.
                    if x > 1 {
                        // We needed at least two more springs, so this question mark must be a spring.
                        // We also need the next character to be a spring, but we handle that by re-adding
                        // x to the remaining arrangements and recursing.
                        remaining_arrangement.push(x);
                        count += count_possibilities(
                            &remaining_row_with_spring,
                            &remaining_arrangement,
                            known_dot_distributions,
                        );
                        known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), count);
                        return count;
                    } else {
                        if x == 1 {
                            // We needed exactly one more spring, so this question mark must be a spring,
                            // and the next character needs to not be a spring.
                            let next_c = remaining_row.pop();
                            if next_c == Some('#') {
                                known_dot_distributions
                                    .insert((row.to_vec(), arrangement.to_vec()), 0);
                                return 0;
                            }
                        }

                        count += count_possibilities(
                            &remaining_row,
                            &remaining_arrangement,
                            known_dot_distributions,
                        );
                        known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), count);
                        return count;
                    }
                }
                None => {
                    // We're not on an active run of springs, and we don't have an opinion about
                    // what's next. We fork.
                    count += count_possibilities(
                        &remaining_row_with_empty,
                        &remaining_arrangement,
                        known_dot_distributions,
                    );
                    count += count_possibilities(
                        &remaining_row_with_spring,
                        &remaining_arrangement,
                        known_dot_distributions,
                    );
                    known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), count);
                    return count;
                }
            }
        }
    }
    count += 1;
    known_dot_distributions.insert((row.to_vec(), arrangement.to_vec()), count);
    count
}

fn process_line(
    line: &str,
    known_dot_distributions: &mut HashMap<(Vec<char>, Vec<u64>), usize>,
) -> usize {
    let split = line.split_whitespace().collect::<Vec<&str>>();
    let mut row = split[0].to_owned();
    row = row.strip_prefix('.').unwrap_or(&row).to_string();
    row = row.strip_suffix('.').unwrap_or(&row).to_string();
    while row.contains("..") {
        row = row.replace("..", ".");
    }
    let row_chars = row.chars().collect::<Vec<char>>();
    let arrangement = split[1]
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    count_possibilities(&row_chars, &arrangement, known_dot_distributions)
}

fn process_line_x5(
    line: &str,
    known_dot_distributions: &mut HashMap<(Vec<char>, Vec<u64>), usize>,
) -> usize {
    let split = line.split_whitespace().collect::<Vec<&str>>();
    let mut row = split[0].to_owned();
    row = format!("{}?{}?{}?{}?{}", row, row, row, row, row);
    row = row.strip_prefix('.').unwrap_or(&row).to_string();
    row = row.strip_suffix('.').unwrap_or(&row).to_string();
    while row.contains("..") {
        row = row.replace("..", ".");
    }
    let row_chars = row.chars().collect::<Vec<char>>();
    let mut arrangement = split[1]
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let base_arrangement = arrangement.clone();
    for _ in 0..4 {
        arrangement.extend(base_arrangement.clone().iter());
    }
    count_possibilities(&row_chars, &arrangement, known_dot_distributions)
}

pub(crate) fn day12() {
    let f: File = File::open("data/day12.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut part1_sum: usize = 0;
    let mut part2_sum: usize = 0;
    let mut known_dot_distributions: HashMap<(Vec<char>, Vec<u64>), usize> = HashMap::new();
    for line in lines {
        part1_sum += process_line(&line, &mut known_dot_distributions);
        part2_sum += process_line_x5(&line, &mut known_dot_distributions);
    }
    println!("Day 12 part 1: {}", part1_sum);
    println!("Day 12 part 2: {}", part2_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line_x5() {
        assert_eq!(process_line_x5("???.### 1,1,3", &mut HashMap::new()), 1);
        assert_eq!(
            process_line_x5(".??..??...?##. 1,1,3", &mut HashMap::new()),
            16384,
        );
        assert_eq!(
            process_line_x5("?#?#?#?#?#?#?#? 1,3,1,6", &mut HashMap::new()),
            1,
        );
        assert_eq!(
            process_line_x5("????.#...#... 4,1,1", &mut HashMap::new()),
            16,
        );
        assert_eq!(
            process_line_x5("????.######..#####. 1,6,5", &mut HashMap::new()),
            2500,
        );
        assert_eq!(
            process_line_x5("?###???????? 3,2,1", &mut HashMap::new()),
            506250,
        );
    }
}
