pub struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn number_of_winning_options(&self) -> u64 {
        let mut count: u64 = 0;
        for time_spent_charging in 1..self.time {
            if (self.time - time_spent_charging) * time_spent_charging > self.record {
                count += 1;
            }
        }
        count
    }
}

pub(crate) fn day06() {
    let part1_races: Vec<Race> = vec![
        Race {
            time: 47,
            record: 400,
        },
        Race {
            time: 98,
            record: 1213,
        },
        Race {
            time: 66,
            record: 1011,
        },
        Race {
            time: 98,
            record: 1540,
        },
    ];
    println!(
        "Day 6 part 1: {}",
        part1_races
            .iter()
            .map(|r| r.number_of_winning_options())
            .product::<u64>()
    );
    println!(
        "Day 6 part 2: {}",
        Race {
            time: 47986698,
            record: 400121310111540
        }
        .number_of_winning_options()
    );
}
