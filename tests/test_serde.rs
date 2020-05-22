use serde::{Deserialize, Serialize};

use dijkstra::graph::{Advance, Graph};

#[test]
fn read_and_search() {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/graph.json");
    let json = std::fs::read_to_string(file).expect("failed to read from json file");
    let mut graph: Graph<State, Props> =
        serde_json::from_str(&json).expect("failed to deserialise graph");
    let path = graph.best_path(0, &[23, 24, 25]).unwrap();
    assert_eq!(path, [72, 98, 6, 79, 94]);
    assert_eq!(graph.edge(path[0]).from, 0);
    assert_eq!(graph.edge(path[0]).to, graph.edge(path[1]).from);
    assert_eq!(graph.edge(path[1]).to, graph.edge(path[2]).from);
    assert_eq!(graph.edge(path[2]).to, graph.edge(path[3]).from);
    assert_eq!(graph.edge(path[3]).to, graph.edge(path[4]).from);
    assert_eq!(graph.edge(path[4]).to, 25);
    let best_target = graph.state(25);
    assert_eq!(best_target.cost(), Some(1.6849905966872787));
}

#[test]
fn make() {
    // a sample graph with 26 nodes (letter of the alphabet)
    // and 100 random edges with uniformly random cost sampled from [0, 1).
    let mut graph: Graph<State, Props> = Graph::new();
    for c in b'a'..=b'z' {
        graph.insert_node(State { name: c as char, cost: None });
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    name: char,
    cost: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Props {
    cost: f64,
}

impl Advance<State, Props> for State {
    fn advance(&self, edge_props: &Props) -> State {
        State {
            name: self.name,
            cost: Some(self.cost.unwrap_or(0.0) + edge_props.cost as f64),
        }
    }
    fn update(&mut self, node_state: State) {
        self.cost = node_state.cost;
    }
    fn cost(&self) -> Option<f64> {
        self.cost
    }
}
