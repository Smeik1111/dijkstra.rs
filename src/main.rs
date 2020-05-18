use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use dijkstra::graph::Graph;

const N: u8 = 10;
const NUM_NODES: usize = 1000;
type Position = (u8, u8, u8);

fn main() {
    // 3d grid with N nodes along each dimension and directed edges to all neighbours
    let mut graph = Graph::new();
    for _ in 0..NUM_NODES {
        graph.insert_node(State {});
    }
    for id in 0..graph.num_nodes() {
        for neighbour in neighbours(position_from(id)) {
            let cost = rand::random::<u8>();
            if cost < 200 {
                // skip the majority of the possible edges
                continue;
            }
            graph.insert_edge(id, id_from(neighbour), Props { cost: cost - 200 });
        }
    }
    let json = serde_json::to_string(&graph).expect("failed to serialise generated graph");
    println!("{}", json);
    let source = id_from((1, 1, 1));
    let targets = vec![
        id_from((N - 2,N - 1, N - 1)),
        id_from((N - 1,N - 2, N - 1)),
        id_from((N - 1,N - 1, N - 2)),
    ];
    if let Some(path) = graph.best_path(source, &targets) {
        eprintln!("{:?}", path);
        eprintln!("{:?}", graph.cost(&path));
    }
    let graph: Graph<State, Props> =
       serde_json::from_str(&json).expect("failed to deserialise generated graph");
}

fn id_from(position: Position) -> usize {
    let i = position.0 as usize;
    let j = position.1 as usize;
    let k = position.2 as usize;
    let n = N as usize;
    n * (n * i + j) + k
}

fn position_from(id: usize) -> Position {
    let n = N as usize;
    let k = (id % n) as u8;
    let j = ((id / n) % n) as u8;
    let i = (id / n / n) as u8;
    (i, j, k)
}

// loopy edges on the sides are allowed
fn neighbours(position: Position) -> Vec<Position> {
    let mut result = Vec::new();
    let range = |index: u8| [index.saturating_sub(1), min(index + 1, N - 1)];
    for &i in range(position.0).iter() {
        for &j in range(position.1).iter() {
            for &k in range(position.2).iter() {
                result.push((i, j, k));
            }
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct State;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Props {
    cost: u8,
}

impl dijkstra::graph::Cost for Props {
    fn cost(&self) -> f64 {
        self.cost as f64
    }
}
