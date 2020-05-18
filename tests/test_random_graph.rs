use dijkstra::graph::Graph;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    name: char,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Props {
    cost: f64,
}

impl dijkstra::graph::Cost for Props {
    fn cost(&self) -> f64 {
        self.cost
    }
}

#[test]
fn read_and_search() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/random_graph.json");
    let json = fs::read_to_string(file).expect("failed to read from json file");
    let graph: Graph<State, Props> =
        serde_json::from_str(&json).expect("failed to deserialise graph");
    let path = graph.best_path(0, &[23, 24, 25]).unwrap();
    assert_eq!(path, [72, 98, 6, 79, 94]);
    assert_eq!(graph.edge(path[0]).from, 0);
    assert_eq!(graph.edge(path[0]).to, graph.edge(path[1]).from);
    assert_eq!(graph.edge(path[1]).to, graph.edge(path[2]).from);
    assert_eq!(graph.edge(path[2]).to, graph.edge(path[3]).from);
    assert_eq!(graph.edge(path[3]).to, graph.edge(path[4]).from);
    assert_eq!(graph.edge(path[4]).to, 25);
    assert_eq!(graph.cost(&path), 1.6849905966872787);
}

#[test]
fn make() {
    // a sample graph with 26 nodes (letter of the alphabet)
    // and 100 random edges with uniformly random cost sampled from [0, 1).
    let mut graph = Graph::new();
    for c in b'a'..=b'z' {
        graph.insert_node(State { name: c as char });
    }
    for _ in 0..100 {
        let from = (rand::random::<u8>() / 10) as usize;
        let to = (rand::random::<u8>() / 10) as usize;
        let cost = rand::random::<f64>();
        graph.insert_edge(from, to, Props { cost: cost });
    }
    let json = serde_json::to_string(&graph).expect("failed to serialise generated graph");
    let graph: Graph<State, Props> =
        serde_json::from_str(&json).expect("failed to deserialise generated graph");
    assert_eq!(graph.num_nodes(), 26);
    assert_eq!(graph.num_edges(), 100);
}
