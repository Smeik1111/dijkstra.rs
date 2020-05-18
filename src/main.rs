use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{thread, time};

use dijkstra::graph::*;

fn main() {
    let (source, targets) = args();
    let graph: Graph<State, Props> =
        serde_json::from_reader(std::io::stdin()).expect("failed to deserialise graph");
    if let Some(path) = graph.best_path(source, &targets) {
        println!("path: {:?}", path);
        println!("cost: {:?}", graph.cost(&path) as u64);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct State;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Props {
    cost: u8,
}

impl Cost for Props {
    fn cost(&self) -> f64 {
        // simulating cost compute time
        thread::sleep(time::Duration::from_millis(10));
        self.cost as f64
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
