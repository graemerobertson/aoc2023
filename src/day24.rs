use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Hailstone {
    x: isize,
    y: isize,
    z: isize,
    x_velocity: isize,
    y_velocity: isize,
    z_velocity: isize,
}

// h1.x + t*h1.x_velocity = h2.x + t'*h2.x_velocity
// h1.y + t*h1.y_velocity = h2.y + t'*h2.y_velocity

// (h1.x + t*h1.x_velocity - h2.x)/h2.x_velocity = t'
// (h1.y + t*h1.y_velocity - h2.y)/h2.y_velocity = t'
// (h1.x + t*h1.x_velocity - h2.x)/h2.x_velocity = (h1.y + t*h1.y_velocity - h2.y)/h2.y_velocity
// (h1.x + t*h1.x_velocity - h2.x)*h2.y_velocity = (h1.y + t*h1.y_velocity - h2.y)*h2.x_velocity
// h1.x*h2.y_velocity + t*h1.x_velocity*h2.y_velocity - h2.x*h2.y_velocity = h1.y*h2.x_velocity + t*h1.y_velocity*h2.x_velocity - h2.y*h2.x_velocity
// t*h1.x_velocity*h2.y_velocity - t*h1.y_velocity*h2.x_velocity = h1.y*h2.x_velocity - h1.x*h2.y_velocity - h2.y*h2.x_velocity + h2.x*h2.y_velocity
// t*(h1.x_velocity*h2.y_velocity - h1.y_velocity*h2.x_velocity) = h1.y*h2.x_velocity - h1.x*h2.y_velocity - h2.y*h2.x_velocity + h2.x*h2.y_velocity
// t = (h1.y*h2.x_velocity - h1.x*h2.y_velocity - h2.y*h2.x_velocity + h2.x*h2.y_velocity)/(h1.x_velocity*h2.y_velocity - h1.y_velocity*h2.x_velocity)
fn paths_intersect(h1: &Hailstone, h2: &Hailstone) -> bool {
    if h1.x_velocity * h2.y_velocity == h1.y_velocity * h2.x_velocity {
        return false;
    }
    let t: f64 = (h1.y * h2.x_velocity - h1.x * h2.y_velocity - h2.y * h2.x_velocity
        + h2.y_velocity * h2.x) as f64
        / (h1.x_velocity * h2.y_velocity - h1.y_velocity * h2.x_velocity) as f64;
    if t < 0.0 {
        return false;
    }
    // Check if t' is in the past
    if ((h1.x as f64 + t * h1.x_velocity as f64 - h2.x as f64) / h2.x_velocity as f64) < 0.0 {
        return false;
    }
    let x = h1.x as f64 + t * h1.x_velocity as f64;
    let y = h1.y as f64 + t * h1.y_velocity as f64;
    (200000000000000.0..=400000000000000.0).contains(&x)
        && (200000000000000.0..=400000000000000.0).contains(&y)
}

pub(crate) fn day24() {
    let f: File = File::open("data/day24.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut hailstones: Vec<Hailstone> = vec![];
    for line in &lines {
        let split = line.split('@').collect::<Vec<&str>>();
        let positions = split[0]
            .split(',')
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let velocities = split[1]
            .split(',')
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        hailstones.push(Hailstone {
            x: positions[0],
            y: positions[1],
            z: positions[2],
            x_velocity: velocities[0],
            y_velocity: velocities[1],
            z_velocity: velocities[2],
        });
    }
    let mut part1_count: usize = 0;
    for (i, hailstone1) in hailstones.iter().enumerate() {
        for hailstone2 in hailstones.iter().skip(i + 1) {
            if paths_intersect(hailstone1, hailstone2) {
                part1_count += 1;
            }
        }
    }
    println!("Day 24 Part 1: {}", part1_count);
}
