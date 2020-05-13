use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::Add;

mod dijkstra;

use dijkstra::Graph;
use serde::{Deserialize, Serialize};

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
        //println!("{}", graph.cost(&path));
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
        let cost = rand::random::<f64>();
        graph.insert_edge(from, to, Props { cost: Cost(cost) });
    }
    graph
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    name: char,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Props {
    cost: Cost,
}

impl dijkstra::Cost for Props {
    type Type = Cost;
    fn cost(&self) -> Self::Type {
        self.cost
    }
    fn zero_cost() -> Self::Type {
        Cost(0.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Cost(f64);

impl Eq for Cost {}
impl Ord for Cost {
    fn cmp(&self, other: &Cost) -> Ordering {
        // panic if any of the values are NaN
        self.partial_cmp(other).unwrap()
    }
}
impl Add for Cost {
    type Output = Cost;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sum for Cost {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Cost>,
    {
        Cost(iter.map(|cost| cost.0).sum())
    }
}
