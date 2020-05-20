use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{thread, time};

use dijkstra::graph::*;

fn main() {
    let (source, targets) = args();
    let mut graph: Graph<State, Props> =
        serde_json::from_reader(std::io::stdin()).expect("failed to deserialise graph");
    graph.state_mut(source).cost = Some(0.0);
    if let Some(path) = graph.best_path(source, &targets) {
        println!("path: {:?}", path);
        let target = graph.edge(*path.last().unwrap()).to;
        println!("cost: {:?}", graph.state(target).cost.unwrap());
    }
}

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

fn args() -> (NodeId, Vec<NodeId>) {
    let matches = App::new("Dijkstra search")
        .arg(
            Arg::with_name("source")
                .long("source")
                .help("Source node id")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("targets")
                .long("targets")
                .help("Target node id(s) (comma separated)")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let source = matches
        .value_of("source")
        .unwrap()
        .parse::<usize>()
        .expect("failed to parse source");
    let targets = matches
        .value_of("targets")
        .unwrap()
        .split(',')
        .map(|target| target.parse::<usize>().expect("failed to parse targets"))
        .collect::<Vec<usize>>();
    (source, targets)
}
