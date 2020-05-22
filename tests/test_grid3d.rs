use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::{thread, time};
use std::cmp::min;

use dijkstra::graph::{Advance, Graph, NodeId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct State {
    cost: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Props {
    cost: u8,
}

impl Advance<State, Props> for State {
    fn advance(&self, edge_props: &Props) -> State {
        // simulating compute time
        thread::sleep(time::Duration::from_millis(10));
        State {
            cost: Some(self.cost.unwrap() + edge_props.cost as f64),
        }
    }
    fn update(&mut self, node_state: State) {
        self.cost = node_state.cost;
    }
    fn cost(&self) -> Option<f64> {
        self.cost
    }
}

// 3d grid with N nodes along each dimension, where each node is connected to all neighbours
const N: usize = 10;

#[test]
fn make_grid3d() {
    let mut graph: Graph<State, Props> = Graph::new();
    let node_ids = (0..N.pow(3))
        .map(|_| graph.insert_node(State { cost: None}))
        .collect::<Vec<_>>();
    for from in node_ids {
        for to in neighbours(from) {
            let cost = rand::random::<u8>();
            graph.insert_edge(from, to, Props { cost: cost });
        }
    }
    let json = serde_json::to_string(&graph).expect("failed to serialise generated graph");
    let graph: Graph<State, Props> =
        serde_json::from_str(&json).expect("failed to deserialise generated graph");
    assert_eq!(graph.num_nodes(), 1000);
    assert_eq!(graph.num_edges(), 6_000);
}

fn neighbours(id: NodeId) -> Vec<NodeId> {
    let position_of = |id| (id % N, (id / N) % N, id / N / N);
    let id_of = |i, j, k| i + N * (j + N * k);
    let less = |index: usize| index.saturating_sub(1);
    let more = |index: usize| min(index + 1, N - 1);
    let (i, j, k) = position_of(id);
    vec![
        id_of(less(i), j, k),
        id_of(more(i), j, k),
        id_of(i, less(j), k),
        id_of(i, more(j), k),
        id_of(i, j, less(k)),
        id_of(i, j, more(k)),
    ]
}
