use std::cmp::Ordering;

use super::NodeId;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum MessageType {
    Flood,
    Bfs,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Message {
    pub time: usize,
    pub sender: NodeId,
    receiver: NodeId,
    pub message_type: MessageType,
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Message {
    pub fn new(time: usize, sender: NodeId, receiver: NodeId, message: MessageType) -> Self {
        Self {
            time,
            sender,
            receiver,
            message,
        }
    }
}
