use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod priority_queue;

// immutable graph, nodes and edges can be added but not deleted
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Graph<State: Debug, Props: Debug> {
    nodes: Vec<Node>,
    states: Vec<State>,
    edges: Vec<Edge>,
    props: Vec<Props>,
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
    fn cost(&self) -> CostType;
}

type NodeId = usize;
type EdgeId = usize;
type CostType = f64;

impl<State: Debug, Props: Debug + Cost> Graph<State, Props> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            states: Vec::new(),
            edges: Vec::new(),
            props: Vec::new(),
        }
    }
    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }
    pub fn state(&self, id: NodeId) -> &State {
        &self.states[id]
    }
    pub fn edge(&self, id: EdgeId) -> &Edge {
        &self.edges[id]
    }
    pub fn props(&self, id: EdgeId) -> &Props {
        &self.props[id]
    }

    pub fn insert_node(&mut self, state: State) -> usize {
        let new_node_id = self.nodes.len();
        self.nodes.push(Node {
            id: new_node_id,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        });
        self.states.push(state);
        new_node_id
    }
    pub fn insert_edge(&mut self, from: usize, to: usize, props: Props) -> usize {
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
    pub fn search(&self, source: NodeId, target: NodeId) -> Option<Vec<EdgeId>> {
        if source == target {
            return Some(Vec::new());
        }
        let mut best_incoming: Vec<Option<BestIncoming>> = vec![None; self.nodes.len()];
        let mut queue = priority_queue::Heap::new();
        let source_cost = 0.0;
        queue.insert(source, source_cost);
        while !queue.is_empty() {
            let (from, from_cost) = queue.extract_min().unwrap();
            for &edge_id in self.nodes[from].outgoing.iter() {
                let to = self.edges[edge_id].to;
                let to_cost = from_cost + self.props[edge_id].cost();
                match best_incoming[to] {
                    Some(BestIncoming(_, cost)) if cost <= to_cost => {
                        continue;
                    }
                    _ => {
                        best_incoming[to] = Some(BestIncoming(edge_id, to_cost));
                        queue.insert(to, to_cost);
                    }
                }
            }
        }
        if best_incoming[target].is_none() {
            return None;
        }
        let mut path: Vec<EdgeId> = Vec::new();
        let mut from = target;
        while from != source {
            if let Some(BestIncoming(edge_id, _)) = best_incoming[from] {
                path.push(edge_id);
                from = self.edges[edge_id].from;
            } else {
                unreachable!();
            }
        }
        path.reverse();
        Some(path)
    }
}

#[derive(Debug, Clone)]
struct BestIncoming(EdgeId, CostType);
