mod dijkstra;
mod types;

use dijkstra::Graph;
use serde::{Deserialize, Serialize};

type CostType = f64;
const ZERO_COST: CostType = 0.0;

fn main() -> Result<(), serde_json::error::Error> {
    // let sample = random_sample();
    // let json = serde_json::to_string(&sample)?;
    // println!("{}", json);

    let graph: Graph<State, Props> = serde_json::from_reader(std::io::stdin())?;
    println!("{:?}", graph.node(25));
    println!("{:?}", graph.state(25));
    println!("{:?}", graph.edge(94));
    println!("{:?}", graph.props(94));
    if let Some(path) = graph.cheapest_path(0, &[23, 24, 25]) {
        println!("{:?}", path);
        println!("{}", graph.cost(&path));
    }
    Ok(())
}

// Sample graph with 26 nodes (letter of the alphabet)
// and 100 random edges with random cost sampled from [0, 255].
pub fn random_sample() -> Graph<State, Props> {
    let mut graph = Graph::new();
    for c in b'a'..=b'z' {
        graph.insert_node(State { name: c as char });
    }
    for _ in 0..100 {
        let from = (rand::random::<u8>() / 10) as usize;
        let to = (rand::random::<u8>() / 10) as usize;
        let cost = rand::random::<CostType>();
        graph.insert_edge(from, to, Props { cost: types::Cost(cost) });
    }
    graph
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    name: char,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Props {
    cost: types::Cost<CostType>,
}

impl dijkstra::Cost for Props {
    type Type = types::Cost<CostType>;
    fn cost(&self) -> Self::Type {
        self.cost
    }
    fn zero_cost() -> Self::Type {
        types::Cost(ZERO_COST)
    }
}
