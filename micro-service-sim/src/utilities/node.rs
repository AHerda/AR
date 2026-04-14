use std::collections::BinaryHeap;

use super::{
    NodeId,
    message::{Message, MessageType},
};

#[derive(Clone, Debug)]
struct NodeState {
    visited: bool,
    parent: Option<NodeId>,
    level: Option<u32>,
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

pub struct Node {
    pub id: NodeId,
    pub neighbors: Vec<NodeId>,
    state: NodeState,
    incoming_messages: BinaryHeap<Message>,
}

impl Node {
    pub fn new(id: NodeId) -> Self {
        Node {
            id,
            neighbors: Vec::new(),
            state: NodeState::default(),
            incoming_messages: BinaryHeap::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: NodeId) {
        self.neighbors.push(neighbor);
    }

    pub fn add_neighbors(&mut self, neighbors: Vec<NodeId>) {
        self.neighbors.extend(neighbors);
    }

    pub fn receive_message(&mut self, sender: NodeId, msg: Message) {
        self.incoming_messages.push(msg);
    }

    fn process_messages_round(&mut self, time: usize) -> Vec<Message> {
        let mut outgoing = Vec::new();

        if self.state.visited {
            return outgoing;
        }

        while self
            .incoming_messages
            .peek()
            .map_or(false, |m| m.time == time)
        {
            let msg = self.incoming_messages.pop().unwrap();
            if
            self.state.visited = true;
            self.state.parent = Some(msg.sender);

            for &neighbor in &self.neighbors {
                if neighbor != self.state.parent.unwrap_or(NodeId::MAX) {
                    outgoing.push(Message::new(time + 1, self.id, neighbor, msg.message_type));
                }
            }
        }

        self.incoming_messages.;
        outgoing
    }
}
