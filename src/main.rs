use clap::{App, Arg};

use dijkstra::advance::{Props, State};
use dijkstra::graph::{Graph, NodeId};

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
