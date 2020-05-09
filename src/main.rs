mod graph;

use graph::Graph;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), serde_json::error::Error> {
    //let sample = random_sample();
    //let json = serde_json::to_string(&sample)?;
    //println!("{}", json);

    let sample: Graph<State, Props> = serde_json::from_reader(std::io::stdin())?;
    println!("{:?}", sample.node(25));
    println!("{:?}", sample.state(25));
    println!("{:?}", sample.edge(99));
    println!("{:?}", sample.props(99));
    let path = sample.search(0, 25);
    println!("{:?}", path);
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
        let cost = rand::random::<u8>();
        graph.insert_edge(from, to, Props { cost });
    }
    graph
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    name: char,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Props {
    cost: u8,
}
