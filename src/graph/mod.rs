use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// immutable graph, nodes and edges can be added but not deleted
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Graph<Data: Debug, Props: Debug> {
    nodes: Vec<Node>,
    data: Vec<Data>,
    edges: Vec<Edge>,
    props: Vec<Props>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Node {
    pub id: NodeId,
    pub incoming: Vec<EdgeId>,
    pub outgoing: Vec<EdgeId>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Edge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
}

type NodeId = usize;
type EdgeId = usize;

impl<Data: Debug, Props: Debug> Graph<Data, Props> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            data: Vec::new(),
            edges: Vec::new(),
            props: Vec::new(),
        }
    }
    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }
    pub fn data(&self, id: NodeId) -> &Data {
        &self.data[id]
    }
    pub fn edge(&self, id: EdgeId) -> &Edge {
        &self.edges[id]
    }
    pub fn props(&self, id: EdgeId) -> &Props {
        &self.props[id]
    }

    pub fn insert_node(&mut self, data: Data) -> usize {
        let new_node_id = self.nodes.len();
        self.nodes.push(Node {
            id: new_node_id,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        });
        self.data.push(data);
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
    pub fn search(&self, _source: NodeId, _target: NodeId) -> Vec<EdgeId> {
        // TODO
        Vec::new()
    }
}
