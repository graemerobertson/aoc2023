use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Game {
    id: usize,
    r_min: usize,
    g_min: usize,
    b_min: usize,
}

pub(crate) fn day02() {
    let f: File = File::open("data/day02.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_lines: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut games: Vec<Game> = vec![];
    for (index, game) in input_lines.iter().enumerate() {
        // Track the maximum number of each colour we see - these are the minimum possible number
        // of that colour in the bag.
        let mut mins: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let split = game.split(&[':', ';']).collect::<Vec<&str>>();
        for s in split {
            if s.contains("Game") {
                continue;
            }
            for colour in s.split(',').collect::<Vec<&str>>() {
                let colour_split: Vec<&str> = colour.trim().split(' ').collect::<Vec<&str>>();
                if mins.get(&colour_split[1]).unwrap() < &colour_split[0].parse::<usize>().unwrap()
                {
                    mins.insert(colour_split[1], colour_split[0].parse::<usize>().unwrap());
                }
            }
        }
        games.push(Game {
            id: index + 1,
            r_min: *mins.get("red").unwrap(),
            g_min: *mins.get("green").unwrap(),
            b_min: *mins.get("blue").unwrap(),
        });
    }

    println!(
        "Day 2 part 1: {}",
        games
            .iter()
            .filter_map(|g| {
                if g.r_min <= 12 && g.g_min <= 13 && g.b_min <= 14 {
                    Some(g.id)
                } else {
                    None
                }
            })
            .sum::<usize>()
    );
    println!(
        "Day 2 part 2: {}",
        games
            .iter()
            .map(|g| g.r_min * g.g_min * g.b_min)
            .sum::<usize>()
    );
}
