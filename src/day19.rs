use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum_of_ratings(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone)]
enum Instruction {
    Accept,
    Reject,
    Workflow(String),
}

struct Criteria {
    value: usize,
    less_than: bool,
    part_component: char,
}

struct Workflow {
    rules: Vec<(Criteria, Instruction)>,
    fallback: Instruction,
}

fn execute_workflow(part: &Part, workflow: &Workflow) -> Instruction {
    for rule in &workflow.rules {
        let criteria = &rule.0;
        let instruction = &rule.1;
        let part_component = criteria.part_component;
        let value = match part_component {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Invalid part component"),
        };
        if (criteria.less_than && value < criteria.value)
            || (!criteria.less_than && value >= criteria.value)
        {
            return instruction.clone();
        }
    }
    workflow.fallback.clone()
}

#[derive(Clone)]
struct Range {
    min_max_vals: HashMap<char, (usize, usize)>,
}

impl Range {
    fn count_parts_in_range(self) -> usize {
        self.min_max_vals
            .iter()
            .map(|(_, (min, max))| max - min + 1)
            .product()
    }

    fn count_acceptable_parts_in_range(
        &mut self,
        entrypoint: String,
        workflows: &HashMap<String, Workflow>,
    ) -> usize {
        let mut count: usize = 0;
        let workflow = workflows.get(&entrypoint).expect("Could not find workflow");
        for rule in &workflow.rules {
            let mut range_that_passes_rule = self.clone();
            let criteria = &rule.0;
            let part_component = criteria.part_component;
            if criteria.less_than {
                range_that_passes_rule
                    .min_max_vals
                    .get_mut(&part_component)
                    .unwrap()
                    .1 = criteria.value - 1;
                self.min_max_vals.get_mut(&part_component).unwrap().0 = criteria.value;
            } else {
                range_that_passes_rule
                    .min_max_vals
                    .get_mut(&part_component)
                    .unwrap()
                    .0 = criteria.value + 1;
                self.min_max_vals.get_mut(&part_component).unwrap().1 = criteria.value;
            }

            match &rule.1 {
                Instruction::Accept => {
                    count += range_that_passes_rule.count_parts_in_range();
                }
                Instruction::Reject => {}
                Instruction::Workflow(w) => {
                    count += range_that_passes_rule
                        .count_acceptable_parts_in_range(w.to_string(), workflows);
                }
            }
        }
        match &workflow.fallback {
            Instruction::Accept => {
                count += self.clone().count_parts_in_range();
            }
            Instruction::Reject => {}
            Instruction::Workflow(w) => {
                count += self.count_acceptable_parts_in_range(w.to_string(), workflows);
            }
        }
        count
    }
}

pub(crate) fn day19() {
    let f: File = File::open("data/day19.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut lines_iter = lines.iter();

    // Pathetically can't work out how to write the code without this line.
    let empty_str = String::default();

    // Absolutely grim parsing code for the workflows, but whatever.
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut line = lines_iter.next().unwrap();
    while !line.is_empty() {
        let split = line.split('{').collect::<Vec<&str>>();
        let name = split[0].to_string();
        let mut split2 = split[1].split(',').collect::<Vec<&str>>();
        let fallback_str = split2.pop().unwrap().replace('}', "").to_string();
        let fallback: Instruction = match fallback_str.as_str() {
            "A" => Instruction::Accept,
            "R" => Instruction::Reject,
            x => Instruction::Workflow(x.to_owned()),
        };
        let mut rules: Vec<(Criteria, Instruction)> = vec![];
        for rule in split2 {
            let split3 = rule.split(':').collect::<Vec<&str>>();
            let instruction_str = split3[1].to_string();
            let instruction: Instruction = match instruction_str.as_str() {
                "A" => Instruction::Accept,
                "R" => Instruction::Reject,
                x => Instruction::Workflow(x.to_owned()),
            };
            let criteria: Criteria = if split3[0].contains('>') {
                let split4 = split3[0].split('>').collect::<Vec<&str>>();
                Criteria {
                    value: split4[1].parse().unwrap(),
                    less_than: false,
                    part_component: split4[0].parse().unwrap(),
                }
            } else {
                let split4 = split3[0].split('<').collect::<Vec<&str>>();
                Criteria {
                    value: split4[1].parse().unwrap(),
                    less_than: true,
                    part_component: split4[0].parse().unwrap(),
                }
            };
            rules.push((criteria, instruction));
        }

        workflows.insert(name, Workflow { rules, fallback });
        line = lines_iter.next().unwrap_or(&empty_str);
    }

    // Parse the parts.
    let parts: Vec<Part> = lines_iter
        .map(|p: &String| {
            serde_json::from_str(
                &p.replace("x=", "\"x\":")
                    .replace("m=", "\"m\":")
                    .replace("a=", "\"a\":")
                    .replace("s=", "\"s\":")
                    .to_owned(),
            )
            .unwrap()
        })
        .collect();

    let mut part1_sum: usize = 0;
    for part in &parts {
        let mut workflow = "in".to_string();
        loop {
            let result = execute_workflow(
                part,
                workflows.get(&workflow).expect("Could not find workflow"),
            );
            match result {
                Instruction::Accept => {
                    part1_sum += part.sum_of_ratings();
                    break;
                }
                Instruction::Reject => {
                    break;
                }
                Instruction::Workflow(w) => workflow = w,
            }
        }
    }
    println!("Day 19 part 1: {}", part1_sum);

    let mut range = Range {
        min_max_vals: HashMap::from([
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]),
    };
    println!(
        "Day 19 part 2: {}",
        range.count_acceptable_parts_in_range("in".to_string(), &workflows)
    );
}
