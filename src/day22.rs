use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Cube {
    x: isize,
    y: isize,
    z: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Brick {
    cubes: HashSet<Cube>,
    min_z: usize,
}

fn fall(bricks: &mut Vec<Brick>) {
    let mut occupied_cells: HashSet<Cube> = HashSet::new();
    for brick in bricks {
        let mut new_min_z: usize = brick.min_z;
        for z in (1..brick.min_z).rev() {
            let new_set_of_cubes = brick
                .cubes
                .iter()
                .map(|cube| Cube {
                    x: cube.x,
                    y: cube.y,
                    z: cube.z - (brick.min_z - z),
                })
                .collect::<HashSet<Cube>>();
            if occupied_cells.is_disjoint(&new_set_of_cubes) {
                new_min_z = z;
            } else {
                break;
            }
        }
        let new_set_of_cubes = brick
            .cubes
            .iter()
            .map(|cube| Cube {
                x: cube.x,
                y: cube.y,
                z: cube.z - (brick.min_z - new_min_z),
            })
            .collect::<HashSet<Cube>>();
        occupied_cells.extend(new_set_of_cubes.clone());
        brick.min_z = new_min_z;
        brick.cubes = new_set_of_cubes;
    }
}

pub(crate) fn day22() {
    let f: File = File::open("data/day22.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut bricks: Vec<Brick> = vec![];
    for line in lines {
        let mut set_of_cubes: HashSet<Cube> = HashSet::new();
        let coords = line.split('~').collect::<Vec<&str>>();
        let coords_start = coords[0]
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let coords_end = coords[1]
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        for x in coords_start[0]..=coords_end[0] {
            for y in coords_start[1]..=coords_end[1] {
                for z in coords_start[2]..=coords_end[2] {
                    set_of_cubes.insert(Cube {
                        x,
                        y,
                        z: z as usize,
                    });
                }
            }
        }
        bricks.push(Brick {
            cubes: set_of_cubes,
            min_z: coords_start[2] as usize,
        });
    }
    bricks.sort_by_key(|b| b.min_z);
    fall(&mut bricks);
    let mut part1_count: usize = 0;
    let mut part2_count: usize = 0;
    for i in 0..bricks.len() {
        let mut bricks_without_i = bricks.clone();
        bricks_without_i.remove(i);
        let mut fallen_bricks_without_i = bricks_without_i.clone();
        fall(&mut fallen_bricks_without_i);
        if fallen_bricks_without_i == bricks_without_i {
            part1_count += 1;
        } else {
            for j in i..bricks_without_i.len() {
                if fallen_bricks_without_i[j] != bricks_without_i[j] {
                    part2_count += 1;
                }
            }
        }
    }

    println!("Day 22 part 1: {:?}", part1_count);
    println!("Day 22 part 2: {:?}", part2_count);
}
