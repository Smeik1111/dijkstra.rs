use serde::{Deserialize, Serialize};
use std::{thread, time};

use crate::graph::Advance;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub cost: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Props {
    pub cost: u8,
}

impl Advance<State, Props> for State {
    fn advance(&self, edge_props: &Props) -> State {
        // simulating compute time
        thread::sleep(time::Duration::from_millis(10));
        State {
            cost: Some(self.cost.unwrap() + edge_props.cost as f64),
        }
    }
    fn update(&mut self, node_state: State) {
        self.cost = node_state.cost;
    }
    fn cost(&self) -> Option<f64> {
        self.cost
    }
}
