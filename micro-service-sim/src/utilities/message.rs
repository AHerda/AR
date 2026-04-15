use rand_distr::{Distribution, Poisson};

use std::cmp::Ordering;

use super::{LAMBDA, NodeId};

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum MessageType {
    Flood,
    Bfs,
}

pub trait Message: Ord + Clone {
    type Time;
    const TimeZero: Self::Time;

    fn to_next_level(&self) -> Self;
    fn next_in_level(&self, receiver: NodeId) -> Self;

    fn new(
        time: Self::Time,
        sender: Option<NodeId>,
        level: usize,
        receiver: NodeId,
        message_type: MessageType,
    ) -> Self;
    fn get_sender(&self) -> Option<NodeId>;
    fn get_receiver(&self) -> NodeId;
    fn get_level(&self) -> usize;
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct MessageSync {
    pub time: usize,
    pub sender: Option<NodeId>,
    pub level: usize,
    pub receiver: NodeId,
    pub message_type: MessageType,
}

impl Ord for MessageSync {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.message_type {
            MessageType::Flood => other.time.cmp(&self.time),
            MessageType::Bfs => other.level.cmp(&self.level),
        }
    }
}

impl PartialOrd for MessageSync {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Message for MessageSync {
    type Time = usize;
    const TimeZero: Self::Time = 0;

    fn new(
        time: Self::Time,
        sender: Option<NodeId>,
        level: usize,
        receiver: NodeId,
        message_type: MessageType,
    ) -> Self {
        Self {
            time,
            sender,
            level,
            receiver,
            message_type,
        }
    }

    fn to_next_level(&self) -> Self {
        Self {
            time: self.time + 1,
            sender: Some(self.receiver),
            level: self.level + 1,
            receiver: self.receiver,
            message_type: self.message_type,
        }
    }

    fn next_in_level(&self, receiver: NodeId) -> Self {
        Self {
            time: self.time,
            sender: self.sender,
            level: self.level,
            receiver,
            message_type: self.message_type,
        }
    }

    fn get_sender(&self) -> Option<NodeId> {
        self.sender
    }

    fn get_receiver(&self) -> NodeId {
        self.receiver
    }

    fn get_level(&self) -> usize {
        self.level
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MessageAsync {
    pub time: f64,
    pub sender: Option<NodeId>,
    pub level: usize,
    pub receiver: NodeId,
    pub message_type: MessageType,
}

impl Eq for MessageAsync {}

impl PartialOrd for MessageAsync {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            other
                .time
                .partial_cmp(&self.time)
                .unwrap_or(Ordering::Equal)
                .then_with(|| other.level.cmp(&self.level)),
        )
    }
}

impl Ord for MessageAsync {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Message for MessageAsync {
    type Time = f64;
    const TimeZero: Self::Time = 0.0;

    fn new(
        time: f64,
        sender: Option<NodeId>,
        level: usize,
        receiver: NodeId,
        message_type: MessageType,
    ) -> Self {
        Self {
            time,
            sender,
            level,
            receiver,
            message_type,
        }
    }

    fn to_next_level(&self) -> Self {
        Self {
            time: self.time,
            sender: Some(self.receiver),
            level: self.level + 1,
            receiver: self.receiver,
            message_type: self.message_type,
        }
    }

    fn next_in_level(&self, receiver: NodeId) -> Self {
        let poi = rand_distr::Poisson::new(LAMBDA).unwrap();
        Self {
            time: self.time + poi.sample(&mut rand::rng()),
            sender: self.sender,
            level: self.level,
            receiver,
            message_type: self.message_type,
        }
    }

    fn get_sender(&self) -> Option<NodeId> {
        self.sender
    }

    fn get_receiver(&self) -> NodeId {
        self.receiver
    }

    fn get_level(&self) -> usize {
        self.level
    }
}
