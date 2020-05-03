use std::fmt::Debug;
use serde::{Deserialize, Serialize};

// immutable graph, nodes and edges can be added but not deleted
#[derive(Debug, Deserialize, Serialize)]
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
    pub fn new() -> Graph<Data, Props> {
        Graph {
            nodes: Vec::new(),
            data: Vec::new(),
            edges: Vec::new(),
            props: Vec::new(),
        }
    }
    pub fn node(self: &Self, id: NodeId) -> &Node {
        &self.nodes[id]
    }
    pub fn data(self: &Self, id: NodeId) -> &Data {
        &self.data[id]
    }
    pub fn edge(self: &Self, id: EdgeId) -> &Edge {
        &self.edges[id]
    }
    pub fn props(self: &Self, id: EdgeId) -> &Props {
        &self.props[id]
    }

    pub fn insert_node(self: &mut Self, data: Data) -> usize {
        let new_node_id = self.nodes.len();
        self.nodes.push(Node {
            id: new_node_id,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        });
        self.data.push(data);
        return new_node_id;
    }
    pub fn insert_edge(self: &mut Self, from: usize, to: usize, props: Props) -> usize {
        let new_edge_id = self.edges.len();
        self.edges.push(Edge {
            id: new_edge_id,
            from: from,
            to: to,
        });
        self.props.push(props);
        self.nodes[from].outgoing.push(new_edge_id);
        self.nodes[to].incoming.push(new_edge_id);
        return new_edge_id;
    }
}
