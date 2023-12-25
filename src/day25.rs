use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Split the edges into sets of disconnected nodes (using a very inefficient algorithm).
fn solve(edges: &[(String, String)]) -> Option<usize> {
    let mut sets: Vec<HashSet<String>> = vec![];
    for edge in edges {
        let mut existing_sets_indices: Vec<usize> = vec![];
        // Find existing sets that contain either node of the edge
        for (index, s) in sets.iter_mut().enumerate() {
            if s.contains(&edge.0) || s.contains(&edge.1) {
                existing_sets_indices.push(index);
                s.insert(edge.0.clone());
                s.insert(edge.1.clone());
            }
        }
        if existing_sets_indices.is_empty() {
            // If there were no existing sets, create a new one
            let mut new_set = HashSet::new();
            new_set.insert(edge.0.clone());
            new_set.insert(edge.1.clone());
            sets.push(new_set.to_owned());
        } else if existing_sets_indices.len() > 1 {
            // If there were multiple existing sets, we need to now combine them
            let combine_at_index = existing_sets_indices.first().unwrap();
            let mut new_set = sets.get(*combine_at_index).unwrap().to_owned();
            for index in existing_sets_indices.iter().rev() {
                if index != combine_at_index {
                    new_set.extend(sets.get(*index).unwrap().clone());
                    sets.remove(*index);
                }
            }
            sets[*combine_at_index] = new_set;
        }
    }
    if sets.len() == 2 {
        Some(sets[0].len() * sets[1].len())
    } else {
        None
    }
}

pub(crate) fn day25() {
    let f: File = File::open("data/day25.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    // Build graph of edges
    let mut edges: Vec<(String, String)> = vec![];
    for line in &lines {
        let split = line.split(": ").collect::<Vec<&str>>();
        for c in split[1].split_whitespace() {
            edges.push((split[0].to_owned(), c.to_owned()));
        }
    }

    // Draw graph using graphviz and find three key edges
    let mut graph = String::from("graph {\n");
    for (a, b) in &edges {
        graph += &format!("  {} -- {};\n", a, b);
    }
    graph += "}";
    // Install graphviz and run the following command to display the graph.
    //   dot -Tsvg -Kneato day25graph.dot > day25graph.svg
    //
    // I've shamelessly stolen this command from someone else so I have no idea what any of the
    // flags do.
    std::fs::write("day25graph.dot", graph).unwrap();

    // Remove the three key edges, determined by inspecting the graph generated above
    for edge in [
        ("vkp", "kfr"),
        ("kfr", "vkp"),
        ("qpp", "vnm"),
        ("vnm", "qpp"),
        ("bff", "rhk"),
        ("rhk", "bff"),
    ] {
        if let Some(index) = edges.iter().position(|x| x.0 == edge.0 && x.1 == edge.1) {
            edges.remove(index);
        }
    }

    // Solve
    println!("Day 25 part 1: {}", solve(&edges).unwrap());
}
