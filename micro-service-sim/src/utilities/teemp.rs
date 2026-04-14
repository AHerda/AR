use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use rand::random_bool;

const P: f64 = 0.1;

type NodeId = usize;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum MessageType {
    Flood,
    Bfs,
}

#[derive(Clone, Eq, PartialEq)]
struct Message {
    time: u64,
    sender: NodeId,
    receiver: NodeId,
    message: MessageType,
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

struct Node {
    id: NodeId,
    neighbors: Vec<NodeId>,
    state: NodeState,
    incoming_messages: BinaryHeap<Message>,
}

impl Node {
    fn new(id: NodeId) -> Self {
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

    fn receive_message(&mut self, sender: NodeId, msg: Message) {
        self.incoming_messages.push(msg);
    }

    fn process_messages(&mut self) -> Vec<(NodeId, Message)> {
        let mut outgoing = Vec::new();

        for (sender, _msg) in &self.incoming_messages {
            if !self.state.visited {
                self.state.visited = true;
                self.state.parent = Some(*sender);

                for &neighbor in &self.neighbors {
                    if neighbor != self.state.parent.unwrap_or(NodeId::MAX) {
                        outgoing.push((neighbor, MessageType::Flood));
                    }
                }
                break;
            }
        }

        self.incoming_messages.clear();
        outgoing
    }
}

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        let mut g = Graph {
            nodes: (0..n).map(|i| Node::new(i)).collect(),
        };

        for i in 0..n {
            g.nodes[i].add_neighbors((0..n).filter(|j| *j != i && random_bool(P)).collect());
        }
        g
    }

    pub fn add_node(&mut self, id: NodeId, neighbors: Vec<NodeId>) {
        self.nodes.insert(id, Node::new(id, neighbors));
    }

    fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    fn all_visited(&self) -> bool {
        self.nodes.values().all(|n| n.state.visited)
    }
}

pub struct SyncSimulator {
    graph: Graph,
    rounds: u64,
    message_count: u64,
}

impl SyncSimulator {
    pub fn new(graph: Graph) -> Self {
        SyncSimulator {
            graph,
            rounds: 0,
            message_count: 0,
        }
    }

    pub fn run(&mut self, start: NodeId) {
        if let Some(node) = self.graph.node_mut(start) {
            node.state.visited = true;
            node.state.level = Some(0);
        }

        let mut current_messages = vec![(start, start, Message::Flood)];

        while !current_messages.is_empty() && !self.graph.all_visited() {
            self.rounds += 1;
            let mut next_messages = Vec::new();

            for (sender, receiver, msg) in current_messages {
                if let Some(node) = self.graph.node_mut(receiver) {
                    node.receive_message(sender, msg);
                }
            }

            for node in self.graph.nodes.values_mut() {
                let outgoing = node.process_messages();
                for (receiver, msg) in outgoing {
                    next_messages.push((node.id, receiver, msg));
                    self.message_count += 1;
                }
            }

            current_messages = next_messages;
        }
    }

    pub fn report(&self) {
        println!("=== Synchronous Flooding ===");
        println!("Rounds: {}", self.rounds);
        println!("Total messages: {}", self.message_count);
        println!("\nNode states:");
        for (id, node) in &self.graph.nodes {
            println!(
                "  Node {}: visited={}, parent={:?}, level={:?}",
                id, node.state.visited, node.state.parent, node.state.level
            );
        }
    }
}

pub struct AsyncSimulator {
    graph: Graph,
    event_queue: BinaryHeap<Event>,
    message_count: u64,
    max_time: u64,
}

impl AsyncSimulator {
    pub fn new(graph: Graph) -> Self {
        AsyncSimulator {
            graph,
            event_queue: BinaryHeap::new(),
            message_count: 0,
            max_time: 0,
        }
    }

    pub fn run(&mut self, start: NodeId) {
        if let Some(node) = self.graph.node_mut(start) {
            node.state.visited = true;
            node.state.level = Some(0);
        }

        if let Some(node) = self.graph.node(start) {
            for &neighbor in &node.neighbors {
                self.event_queue.push(Event {
                    time: 1,
                    sender: start,
                    receiver: neighbor,
                    message: Message::Flood,
                });
            }
        }

        while let Some(event) = self.event_queue.pop() {
            self.max_time = self.max_time.max(event.time);

            if let Some(node) = self.graph.node_mut(event.receiver) {
                if !node.state.visited {
                    node.state.visited = true;
                    node.state.parent = Some(event.sender);

                    for &neighbor in &node.neighbors {
                        if Some(neighbor) != node.state.parent {
                            self.event_queue.push(Event {
                                time: event.time + 1,
                                sender: event.receiver,
                                receiver: neighbor,
                                message: Message::Flood,
                            });
                            self.message_count += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn report(&self) {
        println!("=== Asynchronous Flooding ===");
        println!("Max time: {}", self.max_time);
        println!("Total messages: {}", self.message_count);
        println!("\nNode states:");
        for (id, node) in &self.graph.nodes {
            println!(
                "  Node {}: visited={}, parent={:?}, level={:?}",
                id, node.state.visited, node.state.parent, node.state.level
            );
        }
    }
}
