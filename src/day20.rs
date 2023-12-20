use num_integer::lcm;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

trait Module {
    fn destination_modules(&self) -> &Vec<String>;
    fn insert_input_module(&mut self, _input_module: &str) {
        panic!("Can't insert input module for this module type")
    }
    fn receive_pulse(&mut self, input_module: String, input_pulse: Pulse) -> Vec<(Pulse, String)>;
}

#[derive(Debug, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Default)]
struct Conjunction {
    last_pulses_received: HashMap<String, Pulse>,
    destination_modules: Vec<String>,
}

impl Conjunction {
    fn all_high(&self) -> bool {
        self.last_pulses_received
            .values()
            .all(|p| matches!(p, Pulse::High))
    }
}

impl Module for Conjunction {
    fn destination_modules(&self) -> &Vec<String> {
        &self.destination_modules
    }

    fn insert_input_module(&mut self, input_module: &str) {
        self.last_pulses_received
            .insert(input_module.to_owned(), Pulse::Low);
    }

    fn receive_pulse(&mut self, input_module: String, input_pulse: Pulse) -> Vec<(Pulse, String)> {
        self.last_pulses_received.insert(input_module, input_pulse);
        let output_pulse = if self.all_high() {
            Pulse::Low
        } else {
            Pulse::High
        };
        self.destination_modules
            .clone()
            .into_iter()
            .map(|m: String| (output_pulse, m))
            .collect()
    }
}

#[derive(Default)]
struct FlipFlop {
    on: bool,
    destination_modules: Vec<String>,
}

impl Module for FlipFlop {
    fn destination_modules(&self) -> &Vec<String> {
        &self.destination_modules
    }

    fn receive_pulse(&mut self, _input_module: String, input_pulse: Pulse) -> Vec<(Pulse, String)> {
        if matches!(input_pulse, Pulse::Low) {
            let output_pulse = if self.on { Pulse::Low } else { Pulse::High };
            self.on = !self.on;
            return self
                .destination_modules
                .clone()
                .into_iter()
                .map(|m: String| (output_pulse, m))
                .collect();
        }
        vec![]
    }
}

#[derive(Default)]
struct Broadcaster {
    destination_modules: Vec<String>,
}

impl Module for Broadcaster {
    fn destination_modules(&self) -> &Vec<String> {
        &self.destination_modules
    }

    fn receive_pulse(&mut self, _input_module: String, input_pulse: Pulse) -> Vec<(Pulse, String)> {
        self.destination_modules
            .clone()
            .into_iter()
            .map(|m| (input_pulse, m))
            .collect()
    }
}

pub(crate) fn day20() {
    let f: File = File::open("data/day20.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut rev_module_mapping: HashMap<String, HashSet<String>> = HashMap::new();
    let mut conjunction_modules: Vec<String> = vec![];
    for line in &lines {
        let split = line.split(" -> ").collect::<Vec<&str>>();
        let destination_modules = split[1]
            .split(", ")
            .map(|m| m.into())
            .collect::<Vec<String>>();
        let module_name = if split[0].starts_with("broadcaster") {
            let module_name = "broadcaster".to_string();
            modules.insert(
                module_name.clone(),
                Box::new(Broadcaster {
                    destination_modules: destination_modules.clone(),
                }),
            );
            module_name
        } else if split[0].starts_with('%') {
            let module_name = split[0].strip_prefix('%').unwrap().to_string();
            modules.insert(
                module_name.clone(),
                Box::new(FlipFlop {
                    destination_modules: destination_modules.clone(),
                    ..Default::default()
                }),
            );
            module_name
        } else if split[0].starts_with('&') {
            let module_name = split[0].strip_prefix('&').unwrap().to_string();
            conjunction_modules.push(module_name.clone());
            modules.insert(
                module_name.clone(),
                Box::new(Conjunction {
                    destination_modules: destination_modules.clone(),
                    ..Default::default()
                }),
            );
            module_name
        } else {
            panic!("Unknown module type: {}", line)
        };

        for module in &destination_modules {
            rev_module_mapping
                .entry(module.clone())
                .or_default()
                .insert(module_name.clone());
        }
    }

    for conjunction_module_name in &conjunction_modules {
        for module_name in rev_module_mapping.get(conjunction_module_name).unwrap() {
            modules
                .get_mut(conjunction_module_name)
                .unwrap()
                .insert_input_module(module_name);
        }
    }

    let mut low_pulse_count: usize = 0;
    let mut high_pulse_count: usize = 0;
    let mut button_press_count: usize = 0;

    // By inspection, rx is triggered by a conjunction module called kh, which in turn is triggered
    // by four conjunction modules called pv, qh, xm and hz. So we need all of those modules to
    // send a high pulse at the same time.
    //
    // Soooooooooooo, what I'm going to do is...
    //  - Hardcode those module names
    //  - Find out how many button presses it takes each of them to send a high pulse
    //  - Assume that this stuff all works in cycles (without doing any thinking whatsoever about
    //    whether that's likely to be true).
    //  - Lowest common multiple, bish bash bosh
    let mut graemes_hardcoded_magic_map: HashMap<String, usize> = HashMap::from([
        ("pv".into(), 0),
        ("qh".into(), 0),
        ("xm".into(), 0),
        ("hz".into(), 0),
    ]);
    'outer: loop {
        button_press_count += 1;
        let mut active_pulses: Vec<(Pulse, String, String)> =
            vec![(Pulse::Low, "broadcaster".into(), "human".into())];
        if button_press_count <= 1000 {
            low_pulse_count += 1;
        }

        loop {
            let mut next_pulses: Vec<(Pulse, String, String)> = vec![];
            for pulse in active_pulses {
                if let Some(next_module) = modules.get_mut(&pulse.1) {
                    next_pulses.extend(
                        next_module
                            .receive_pulse(pulse.2, pulse.0)
                            .iter()
                            .map(|p| (p.0, p.1.clone(), pulse.1.clone())),
                    )
                }
            }
            if next_pulses.is_empty() {
                break;
            } else {
                for (k, v) in graemes_hardcoded_magic_map.iter_mut() {
                    if next_pulses
                        .iter()
                        .any(|p| matches!(p.0, Pulse::High) && p.2 == *k && v == &0)
                    {
                        *v = button_press_count;
                    }
                }
                if graemes_hardcoded_magic_map.values().all(|v| v != &0) {
                    break 'outer;
                }
                if button_press_count <= 1000 {
                    low_pulse_count += next_pulses
                        .iter()
                        .filter(|p| matches!(p.0, Pulse::Low))
                        .count();
                    high_pulse_count += next_pulses
                        .iter()
                        .filter(|p| matches!(p.0, Pulse::High))
                        .count();
                }
                active_pulses = next_pulses;
            }
        }
    }
    println!("Day 20 part 1: {} ", low_pulse_count * high_pulse_count);
    println!(
        "Day 20 part 2: {} ",
        graemes_hardcoded_magic_map
            .values()
            .fold(1, |a, b| lcm(a, *b))
    );
}
