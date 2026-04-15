use std::collections::BinaryHeap;

use crate::utilities::message::Message;

use super::{
    NodeId,
    message::{MessageSync, MessageType},
};

#[derive(Clone, Debug)]
pub struct NodeState {
    pub visited: bool,
    pub parent: Option<NodeId>,
    pub level: Option<usize>,
}

impl Default for NodeState {
    fn default() -> Self {
        NodeState {
            visited: false,
            parent: None,
            level: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node<T: Message> {
    pub id: NodeId,
    pub neighbors: Vec<NodeId>,
    pub state: NodeState,
    incoming_messages: BinaryHeap<T>,
}

impl<T: Message> Node<T> {
    pub fn new(id: NodeId) -> Self {
        Node {
            id,
            neighbors: Vec::new(),
            state: NodeState::default(),
            incoming_messages: BinaryHeap::new(),
        }
    }

    pub fn get_bfs_data(&self) -> (NodeId, usize) {
        (
            self.state.parent.unwrap_or(404),
            self.state
                .level
                .expect("everyone should have a level (maybe not fully connected graph)"),
        )
    }

    pub fn add_neighbor(&mut self, neighbor: NodeId) {
        self.neighbors.push(neighbor);
    }

    pub fn add_neighbors(&mut self, neighbors: Vec<NodeId>) {
        self.neighbors.extend(neighbors);
    }

    pub fn receive_message(&mut self, msg: T) {
        self.incoming_messages.push(msg);
    }

    pub fn process_messages_round(&mut self) -> Vec<T> {
        let mut outgoing = Vec::new();

        if self.state.visited {
            return outgoing;
        }

        if !self.incoming_messages.is_empty() {
            let mut msg = self.incoming_messages.pop().unwrap();
            self.state.visited = true;
            self.state.parent = msg.get_sender();
            self.state.level = Some(msg.get_level());

            msg = msg.to_next_level();

            for &neighbor in &self.neighbors {
                if neighbor != self.state.parent.unwrap_or(NodeId::MAX) {
                    msg = msg.next_in_level(neighbor);
                    outgoing.push(msg.clone());
                }
            }
        }

        outgoing
    }
}
