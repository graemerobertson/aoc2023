use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    previous_direction: Direction,
    distance_travelled_in_stright_line: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
    direction: Direction,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    adj_list: &[Vec<Edge>],
    start: usize,
    goal: usize,
    straight_line_constraints: (usize, usize),
) -> Option<usize> {
    // dist[node] = map of current shortest distances from `start` to `node` arriving at node
    // where we've arrived at `node` by travelling in a specific distance for a specific amount of
    // time.
    let mut dist: Vec<_> = (0..adj_list.len())
        .map(|_| {
            let mut max_costs_map: HashMap<(Direction, usize), usize> = HashMap::new();
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                for distance in 0..straight_line_constraints.1 {
                    max_costs_map.insert((direction, distance + 1), usize::MAX);
                }
            }
            max_costs_map
        })
        .collect();

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start,
        previous_direction: Direction::Start,
        distance_travelled_in_stright_line: 0,
    });

    let mut best_costs: Vec<usize> = vec![];

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        previous_direction,
        distance_travelled_in_stright_line,
    }) = heap.pop()
    {
        if position == goal {
            // We've reached our goal, but I _think_ we might still have better paths available
            // on our heap due to the straight line constraints. Not sure... Either way, there's
            // little harm in processing the rest of the existing heap.
            if distance_travelled_in_stright_line >= straight_line_constraints.0 {
                best_costs.push(cost);
            }
            continue;
        }

        // We've already found a better way here.
        if previous_direction != Direction::Start
            && cost
                > *dist[position]
                    .get(&(previous_direction, distance_travelled_in_stright_line))
                    .unwrap()
        {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            // Don't go backwards
            if edge.direction == Direction::Up && previous_direction == Direction::Down
                || edge.direction == Direction::Down && previous_direction == Direction::Up
                || edge.direction == Direction::Left && previous_direction == Direction::Right
                || edge.direction == Direction::Right && previous_direction == Direction::Left
            {
                continue;
            }

            let new_distance_travelled_in_stright_line = if edge.direction == previous_direction {
                distance_travelled_in_stright_line + 1
            } else {
                1
            };

            // If we've changed direction without hitting our min straight line constraint,
            // continue.
            //
            // If we've exceded our max straight line constraint, continue.
            if (previous_direction != Direction::Start
                && new_distance_travelled_in_stright_line == 1
                && distance_travelled_in_stright_line < straight_line_constraints.0)
                || new_distance_travelled_in_stright_line > straight_line_constraints.1
            {
                continue;
            }

            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
                previous_direction: edge.direction,
                distance_travelled_in_stright_line: new_distance_travelled_in_stright_line,
            };

            // If so, add it to the frontier and continue
            if next.cost
                < *dist[next.position]
                    .get(&(edge.direction, new_distance_travelled_in_stright_line))
                    .unwrap()
            {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist[next.position]
                    .get_mut(&(edge.direction, new_distance_travelled_in_stright_line))
                    .unwrap() = next.cost;
            }
        }
    }

    if !best_costs.is_empty() {
        return Some(*best_costs.iter().min().unwrap());
    }

    // Goal not reachable
    None
}

fn build_graph(
    grid: &[Vec<usize>],
    number_of_rows: usize,
    number_of_columns: usize,
    graph: &mut Vec<Vec<Edge>>,
) {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, _) in row.iter().enumerate() {
            let mut edges: Vec<Edge> = Vec::new();
            if row_index > 0 {
                edges.push(Edge {
                    node: number_of_columns * (row_index - 1) + column_index,
                    cost: grid[row_index - 1][column_index],
                    direction: Direction::Up,
                });
            }
            if column_index > 0 {
                edges.push(Edge {
                    node: number_of_columns * (row_index) + column_index - 1,
                    cost: grid[row_index][column_index - 1],
                    direction: Direction::Left,
                });
            }
            if row_index < number_of_rows - 1 {
                edges.push(Edge {
                    node: number_of_columns * (row_index + 1) + column_index,
                    cost: grid[row_index + 1][column_index],
                    direction: Direction::Down,
                });
            }
            if column_index < number_of_columns - 1 {
                edges.push(Edge {
                    node: number_of_columns * (row_index) + column_index + 1,
                    cost: grid[row_index][column_index + 1],
                    direction: Direction::Right,
                });
            }
            graph.push(edges);
        }
    }
}

pub(crate) fn day17() {
    let f: File = File::open("data/day17.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let number_of_rows: usize = input_data.len();
    let number_of_columns: usize = input_data.get(0).unwrap().len();
    let mut grid: Vec<Vec<usize>> = vec![vec![0; number_of_columns]; number_of_rows];
    let start_point: usize = 0;
    let end_point: usize = number_of_columns * number_of_rows - 1;

    for (i, line) in input_data.iter().enumerate() {
        for (j, point) in line.chars().enumerate() {
            grid[i][j] = usize::try_from(point.to_digit(10).unwrap()).unwrap();
        }
    }

    let mut graph: Vec<Vec<Edge>> = Vec::new();
    build_graph(&grid, number_of_rows, number_of_columns, &mut graph);

    println!(
        "Day 17 part 1: {}",
        shortest_path(&graph, start_point, end_point, (1, 3)).unwrap()
    );
    println!(
        "Day 17 part 2: {}",
        shortest_path(&graph, start_point, end_point, (4, 10)).unwrap()
    );
}
