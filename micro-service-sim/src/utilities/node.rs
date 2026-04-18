use std::cmp::Ordering;
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
    time_of_last_sent_msg: T::Time,
    acks_gotten: usize,
}

impl<T: Message> Node<T> {
    pub fn new(id: NodeId) -> Self {
        Node {
            id,
            neighbors: Vec::new(),
            state: NodeState::default(),
            incoming_messages: BinaryHeap::new(),
            time_of_last_sent_msg: T::TimeZero,
            acks_gotten: 0,
        }
    }

    pub fn got_all_acks(&self) -> bool {
        match self.state.parent {
            Some(_) => self.acks_gotten + 1 == self.neighbors.len(),
            None => self.acks_gotten == self.neighbors.len(),
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

    pub fn get_neighbors(&self) -> Vec<NodeId> {
        self.neighbors.clone()
    }

    pub fn receive_message(&mut self, msg: T) {
        self.incoming_messages.push(msg);
    }

    pub fn process_messages_round(&mut self) -> Vec<T> {
        let mut outgoing = Vec::new();

        if !self.incoming_messages.is_empty() {
            let mut msg = self.incoming_messages.pop().unwrap();
            if msg.is_ack() {
                self.acks_gotten += 1;
                if self.got_all_acks() {
                    self.time_of_last_sent_msg = msg.get_time();
                    if let Some(parent) = self.state.parent {
                        outgoing.push(msg.create_ack(self.time_of_last_sent_msg.clone(), parent));
                        self.time_of_last_sent_msg = outgoing[0].get_time();
                    }
                }
                return outgoing;
            } else if self.state.visited {
                outgoing.push(msg.create_ack(
                    self.time_of_last_sent_msg.clone(),
                    msg.get_sender().unwrap(),
                ));
                self.time_of_last_sent_msg = outgoing[0].get_time();
                return outgoing;
            }

            self.state.visited = true;
            self.state.parent = msg.get_sender();
            self.state.level = Some(msg.get_level());

            if self.neighbors.len() == 1 && msg.get_sender().is_some() {
                outgoing.push(msg.create_ack(
                    self.time_of_last_sent_msg.clone(),
                    msg.get_sender().expect("root only got one neighbor"),
                ));
                return outgoing;
            }

            msg = msg.to_next_level(self.time_of_last_sent_msg.clone());

            for &neighbor in &self.neighbors {
                if neighbor != self.state.parent.unwrap_or(NodeId::MAX) {
                    msg = msg.next_in_level(neighbor);
                    outgoing.push(msg.clone());
                }
            }

            if !outgoing.is_empty() {
                self.time_of_last_sent_msg = outgoing.last().unwrap().get_time();
            }
        }

        outgoing
    }
}
