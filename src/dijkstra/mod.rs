use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::Debug;

mod priority_queue;

// immutable graph, nodes and edges can be added but not deleted
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Graph<NodeState: Debug, EdgeProps: Debug> {
    nodes: Vec<Node>,
    states: Vec<NodeState>,
    edges: Vec<Edge>,
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
    fn cost(&self) -> CostType;
}

type NodeId = usize;
type EdgeId = usize;
type CostType = f64;

impl<NodeState: Debug, EdgeProps: Debug + Cost> Graph<NodeState, EdgeProps> {
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
    pub fn state(&self, id: NodeId) -> &NodeState {
        &self.states[id]
    }
    pub fn edge(&self, id: EdgeId) -> &Edge {
        &self.edges[id]
    }
    pub fn props(&self, id: EdgeId) -> &EdgeProps {
        &self.props[id]
    }
    pub fn cost(&self, path: &[EdgeId]) -> CostType {
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
    pub fn cheapest_path(&self, source: NodeId, targets: &[NodeId]) -> Option<Vec<EdgeId>> {
        if targets.contains(&source) {
            return Some(Vec::new());
        }
        let mut best_incoming: Vec<Option<Incoming>> = vec![None; self.nodes.len()];
        let mut is_closed: Vec<bool> = vec![false; self.nodes.len()];
        let mut queue = priority_queue::Heap::new();
        let source_cost = 0.0;
        queue.insert(source, source_cost);
        while !queue.is_empty() {
            let (from, from_cost) = queue.extract_min().unwrap();
            is_closed[from] = true;
            for &edge_id in self.nodes[from].outgoing.iter() {
                let to = self.edges[edge_id].to;
                if is_closed[to] {
                    continue;
                }
                let to_cost = from_cost + self.props[edge_id].cost();
                let incoming = Incoming(edge_id, to_cost);
                if best_incoming[to].is_none() || incoming < best_incoming[to].unwrap() {
                    best_incoming[to] = Some(incoming);
                    queue.insert(to, to_cost);
                }
            }
        }
        let cheapest_target: Option<NodeId> = targets
            .iter()
            .cloned()
            .filter(|&target| best_incoming[target].is_some())
            .min_by_key(|&target| best_incoming[target].unwrap());
        let mut node_id = cheapest_target?;
        let mut path = Vec::new();
        while node_id != source {
            if let Some(Incoming(edge_id, _)) = best_incoming[node_id] {
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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Incoming(EdgeId, CostType);
impl PartialOrd for Incoming {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}
impl Eq for Incoming {}
impl Ord for Incoming {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(&other) {
            Some(Ordering::Less) => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}
