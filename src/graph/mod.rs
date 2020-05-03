use std::fmt::{Debug, Display};

// immutable graph, nodes and edges can be added but not deleted
#[derive(Debug)]
pub struct Graph<Data: Debug, Props: Debug> {
    nodes: Vec<Node>,
    data: Vec<Data>,
    edges: Vec<Edge>,
    props: Vec<Props>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: usize,
    pub incoming: Vec<usize>,
    pub outgoing: Vec<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: usize,
    pub from: usize,
    pub to: usize,
}

impl<Data: Debug, Props: Debug> Graph<Data, Props> {
    pub fn new() -> Graph<Data, Props> {
        Graph {
            nodes: Vec::new(),
            data: Vec::new(),
            edges: Vec::new(),
            props: Vec::new(),
        }
    }
    pub fn node(self: &Self, node_id: usize) -> &Node {
        &self.nodes[node_id]
    }
    pub fn data(self: &Self, node_id: usize) -> &Data {
        &self.data[node_id]
    }
    pub fn edge(self: &Self, edge_id: usize) -> &Edge {
        &self.edges[edge_id]
    }
    pub fn props(self: &Self, edge_id: usize) -> &Props {
        &self.props[edge_id]
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
    pub fn json(&self) -> String
    where
        Data: Display,
        Props: Display,
    {
        let mut result = "{\"nodes\":[".to_string();
        for id in 0..self.nodes.len() {
            result += &format!("{}", self.data[id]);
            if id < self.nodes.len() - 1 {
                result += &",";
            }
        }
        result += &"],";
        result += &"\"edges\":[";
        for id in 0..self.edges.len() {
            let edge = &self.edges[id];
            let props = &self.props[id];
            let from = &self.nodes[edge.from];
            let to = &self.nodes[edge.to];
            result += &format!(
                "{{\"from\":{},\"to\":{},\"props\":{}}}",
                self.data[from.id], self.data[to.id], props
            );
            if id < self.edges.len() - 1 {
                result += &",";
            }
        }
        result += &"]}";
        result
    }
}
