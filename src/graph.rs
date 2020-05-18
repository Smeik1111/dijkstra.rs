use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::priority_queue;

// data-oriented graph with user-defined node states and edge props;
// nodes and edges can be inserted but not deleted
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Graph<NodeState: Debug, EdgeProps: Debug> {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    states: Vec<NodeState>,
    props: Vec<EdgeProps>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Node {
    pub id: NodeId,
    pub incoming: Vec<EdgeId>,
    pub outgoing: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
}

pub trait Cost {
    fn cost(&self) -> f64;
}

pub type NodeId = usize;
pub type EdgeId = usize;

impl<NodeState: Debug, EdgeProps: Debug + Cost> Graph<NodeState, EdgeProps> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            states: Vec::new(),
            props: Vec::new(),
        }
    }
    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }
    pub fn edge(&self, id: EdgeId) -> &Edge {
        &self.edges[id]
    }
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
    pub fn state(&self, id: NodeId) -> &NodeState {
        &self.states[id]
    }
    pub fn props(&self, id: EdgeId) -> &EdgeProps {
        &self.props[id]
    }
    pub fn cost(&self, path: &[EdgeId]) -> f64 {
        path.iter()
            .cloned()
            .map(|edge_id| self.props[edge_id].cost())
            .sum()
    }
    pub fn insert_node(&mut self, state: NodeState) -> NodeId {
        let new_node_id = self.nodes.len();
        self.nodes.push(Node {
            id: new_node_id,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        });
        self.states.push(state);
        new_node_id
    }
    pub fn insert_edge(&mut self, from: NodeId, to: NodeId, props: EdgeProps) -> EdgeId {
        let new_edge_id = self.edges.len();
        self.edges.push(Edge {
            id: new_edge_id,
            from,
            to,
        });
        self.props.push(props);
        self.nodes[from].outgoing.push(new_edge_id);
        self.nodes[to].incoming.push(new_edge_id);
        new_edge_id
    }
    // find the cheapest path to any of the targets
    pub fn best_path(&self, source: NodeId, targets: &[NodeId]) -> Option<Vec<EdgeId>> {
        if targets.contains(&source) {
            return Some(Vec::new());
        }
        // from the source, use breadth-first search to find the cheapest incoming edge for each node
        let mut best_incoming = vec![None; self.nodes.len()];
        let mut best_cost = vec![None; self.nodes.len()];
        let mut best_target = None;
        let mut is_closed = vec![false; self.nodes.len()];
        let mut queue = priority_queue::Heap::<f64>::new();
        queue.insert(source, 0.0);
        while !queue.is_empty() {
            let (from, from_cost) = queue.extract_min().unwrap();
            if targets.contains(&from) {
                // all other targets are going to be more expensive, since we're using priority queue
                best_target = Some(from);
                break;
            }
            is_closed[from] = true;
            for (edge_id, edge_cost) in self.nodes[from]
                .outgoing
                .iter()
                .filter(|&&id| self.edges[id].to != from)
                .filter(|&&id| !is_closed[self.edges[id].to])
                .map(|&id| (id, self.props[id].cost()))
            {
                let to = self.edges[edge_id].to;
                let to_cost = from_cost + edge_cost;
                if best_cost[to].is_none() || to_cost < best_cost[to].unwrap() {
                    best_cost[to] = Some(to_cost);
                    best_incoming[to] = Some(edge_id);
                    queue.insert(to, to_cost);
                    // the queue might still have the old more expensive items for 'to',
                    // but they will be discarded when they eventually get to the front of the queue
                }
            }
        }
        // then find the cheapest path walking back from the cheapest target via the cheapest incoming edges
        let mut node_id = best_target?;
        let mut path = Vec::new();
        while node_id != source {
            if let Some(edge_id) = best_incoming[node_id] {
                path.push(edge_id);
                node_id = self.edges[edge_id].from;
            } else {
                unreachable!();
            }
        }
        path.reverse();
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(graph.node(a).incoming.is_empty());
        assert_eq!(graph.node(a).outgoing, [ab, ad]);
        assert_eq!(graph.state(a).name, 'a');

        assert_eq!(graph.node(b).id, b);
        assert_eq!(graph.node(b).incoming, [ab]);
        assert_eq!(graph.node(b).outgoing, [bc]);
        assert_eq!(graph.state(b).name, 'b');

        assert_eq!(graph.node(c).id, c);
        assert_eq!(graph.node(c).incoming, [bc, dc]);
        assert!(graph.node(c).outgoing.is_empty());
        assert_eq!(graph.state(c).name, 'c');

        assert_eq!(graph.node(d).id, d);
        assert_eq!(graph.node(d).incoming, [ad]);
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
}
