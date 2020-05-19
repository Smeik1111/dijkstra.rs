use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::Sync;

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
    pub outgoing: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
}

pub type NodeId = usize;
pub type EdgeId = usize;

pub trait Cost {
    fn cost(&self) -> f64;
}


impl<NodeState: Debug + Sync, EdgeProps: Debug + Cost + Sync> Graph<NodeState, EdgeProps> {
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
                .par_iter()
                .filter(|&&id| self.edges[id].to != from && !is_closed[self.edges[id].to])
                .map(|&id| (id, self.props[id].cost()))
                .collect::<Vec<_>>()
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
