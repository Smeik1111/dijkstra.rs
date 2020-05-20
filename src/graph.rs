use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::priority_queue;

// data-oriented graph with user-defined node states and edge props;
// nodes and edges can be inserted but not deleted
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Graph<NodeState, EdgeProps> {
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

// NodeState has to implement this trait
pub trait Advance<NodeState, EdgeProps> {
    fn advance(&self, edge_props: &EdgeProps) -> NodeState;
    fn update(&mut self, node_state: NodeState);
    fn cost(&self) -> Option<f64>;
}

impl<NodeState, EdgeProps> Graph<NodeState, EdgeProps> 
where
    NodeState: Sync + Send + Advance<NodeState, EdgeProps>,
    EdgeProps: Sync
{
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            states: Vec::new(),
            props: Vec::new(),
        }
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
    pub fn best_path(&mut self, source: NodeId, targets: &[NodeId]) -> Option<Vec<EdgeId>> {
        if targets.contains(&source) {
            return Some(Vec::new());
        }
        // from the source, use breadth-first search to find the cheapest incoming edge for each node
        let mut best_incoming = vec![None; self.nodes.len()];
        let mut best_target = None;
        let mut is_closed = vec![false; self.nodes.len()];
        let mut queue = priority_queue::Heap::<f64>::new();
        queue.insert(source, self.states[source].cost().unwrap_or(0.0));
        while !queue.is_empty() {
            let (from, _) = queue.extract_min().unwrap();
            if targets.contains(&from) {
                // all other targets are going to be more expensive, since we're using priority queue
                best_target = Some(from);
                break;
            }
            is_closed[from] = true;
            let outgoing_edges = self.nodes[from]
                .outgoing
                .iter()
                .cloned()
                .filter(|&edge_id| self.edges[edge_id].to != from && !is_closed[self.edges[edge_id].to])
                .collect::<Vec<_>>();
            for (edge_id, state) in outgoing_edges
                .par_iter()
                .map(|&edge_id| (edge_id, self.states[from].advance(&self.props[edge_id])))
                .collect::<Vec<_>>()
            {
                let to = self.edges[edge_id].to;
                let cost = state.cost().unwrap();
                if let Some(old_cost) = self.states[to].cost() {
                    if old_cost <= cost {
                        continue;
                    }
                }
                self.states[to].update(state);
                best_incoming[to] = Some(edge_id);
                queue.insert(to, cost);
                // the queue might still have the old more expensive items for 'to',
                // but they will be discarded when they eventually get to the front of the queue
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
    pub fn state_mut(&mut self, id: NodeId) -> &mut NodeState {
        &mut self.states[id]
    }
    pub fn props(&self, id: EdgeId) -> &EdgeProps {
        &self.props[id]
    }
    pub fn props_mut(&mut self, id: EdgeId) -> &mut EdgeProps {
        &mut self.props[id]
    }
}
