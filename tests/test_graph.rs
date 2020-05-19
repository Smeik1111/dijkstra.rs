use serde::{Deserialize, Serialize};

use dijkstra::graph::{Cost, Graph};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct State {
    name: char,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Props {
    cost: u8,
}

impl Cost for Props {
    fn cost(&self) -> f64 {
        self.cost as f64
    }
}

#[test]
fn node_state() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });
    let c = graph.insert_node(State { name: 'c' });
    let d = graph.insert_node(State { name: 'd' });

    let ab = graph.insert_edge(a, b, Props { cost: 1 });
    let bc = graph.insert_edge(b, c, Props { cost: 1 });
    let ad = graph.insert_edge(a, d, Props { cost: 1 });
    let dc = graph.insert_edge(d, c, Props { cost: 1 });

    assert_eq!(graph.node(a).id, a);
    assert_eq!(graph.node(a).outgoing, [ab, ad]);
    assert_eq!(graph.state(a).name, 'a');

    assert_eq!(graph.node(b).id, b);
    assert_eq!(graph.node(b).outgoing, [bc]);
    assert_eq!(graph.state(b).name, 'b');

    assert_eq!(graph.node(c).id, c);
    assert!(graph.node(c).outgoing.is_empty());
    assert_eq!(graph.state(c).name, 'c');

    assert_eq!(graph.node(d).id, d);
    assert_eq!(graph.node(d).outgoing, [dc]);
    assert_eq!(graph.state(d).name, 'd');
}

#[test]
fn edge_props() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });

    let ab = graph.insert_edge(a, b, Props { cost: 1 });

    assert_eq!(graph.edge(ab).id, ab);
    assert_eq!(graph.edge(ab).from, a);
    assert_eq!(graph.edge(ab).to, b);
    assert_eq!(graph.props(ab).cost, 1);
}

#[test]
fn best_path() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });
    let c = graph.insert_node(State { name: 'c' });
    let d = graph.insert_node(State { name: 'd' });

    graph.insert_edge(a, b, Props { cost: 1 });
    graph.insert_edge(b, c, Props { cost: 90 });
    let ad = graph.insert_edge(a, d, Props { cost: 10 });
    let dc = graph.insert_edge(d, c, Props { cost: 20 });
    graph.insert_edge(d, b, Props { cost: 1 });

    // three paths are possible from a to c: ab-bc, ad-db-bc, and ad-dc
    let path = graph.best_path(a, &[c]).unwrap();

    assert_eq!(path, [ad, dc]);
    assert_eq!(graph.cost(&path), 30.0);
}

#[test]
fn fork() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });
    let c = graph.insert_node(State { name: 'c' });

    graph.insert_edge(a, b, Props { cost: 2 });
    let ac = graph.insert_edge(a, c, Props { cost: 1 });

    let path = graph.best_path(a, &[b, c]).unwrap();

    assert_eq!(path, [ac]);
    assert_eq!(graph.cost(&path), 1.0);
}

#[test]
fn chain() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });
    let c = graph.insert_node(State { name: 'c' });

    let ab = graph.insert_edge(a, b, Props { cost: 2 });
    graph.insert_edge(b, c, Props { cost: 1 });

    let path = graph.best_path(a, &[b, c]).unwrap();

    assert_eq!(path, [ab]);
    assert_eq!(graph.cost(&path), 2.0);
}

#[test]
fn multi_edge() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });

    let u = graph.insert_edge(a, b, Props { cost: 3 });
    let v = graph.insert_edge(a, b, Props { cost: 2 });
    let w = graph.insert_edge(a, b, Props { cost: 1 });

    assert_ne!(u, v);
    assert_ne!(u, w);
    assert_ne!(v, w);

    let path = graph.best_path(a, &[b]).unwrap();

    assert_eq!(path, [w]);
    assert_eq!(graph.cost(&path), 1.0);
}

#[test]
fn loopy_edge() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });

    let u = graph.insert_edge(a, a, Props { cost: 1 });
    let v = graph.insert_edge(a, b, Props { cost: 2 });

    assert_ne!(u, v);

    let path = graph.best_path(a, &[b]).unwrap();

    assert_eq!(path, [v]);
    assert_eq!(graph.cost(&path), 2.0);
}

#[test]
fn disconnected() {
    let mut graph: Graph<State, Props> = Graph::new();
    let a = graph.insert_node(State { name: 'a' });
    let b = graph.insert_node(State { name: 'b' });

    let path = graph.best_path(a, &[b]);
    assert!(path.is_none());
}
