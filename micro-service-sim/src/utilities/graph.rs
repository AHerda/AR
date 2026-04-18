use rand::{RngExt, seq::IndexedRandom};
use std::{collections::VecDeque, fmt::Display};

use super::{
    NodeId, P,
    message::{Message, MessageAsync, MessageSync, MessageType},
    node::Node,
};

#[derive(Clone, Debug)]
pub struct Graph<T: Message> {
    nodes: Vec<Node<T>>,
    pub messages_sent: usize,
}

impl<T: Message> Graph<T> {
    pub fn new(n: usize) -> Self {
        let mut g = Graph {
            nodes: (0..n).map(|i| Node::new(i)).collect(),
            messages_sent: 0,
        };
        let mut rng = rand::rng();

        let mut are_connected = vec![false; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if rng.random_bool(P) {
                    g.nodes[i].add_neighbor(j);
                    g.nodes[j].add_neighbor(i);
                    are_connected[i] = true;
                    are_connected[j] = true;
                }
            }
        }

        for (i, connected) in are_connected.iter().enumerate() {
            if !connected {
                let mut target = rng.random_range(..n);
                while target == i {
                    target = rng.random_range(..n);
                }
                g.nodes[target].add_neighbor(i);
                g.nodes[i].add_neighbor(target);
            }
        }

        let mut bfs_levels = g.try_get_bfs_tree_levels();
        while bfs_levels.iter().any(|node| node.is_none()) {
            let (connected, not_connected): (Vec<_>, Vec<_>) =
                bfs_levels.iter().enumerate().partition(|n| n.1.is_some());

            let to_connect_1 = connected.choose(&mut rng).unwrap().0;
            let to_connect_2 = not_connected.choose(&mut rng).unwrap().0;
            g.nodes[to_connect_1].add_neighbor(to_connect_2);
            g.nodes[to_connect_2].add_neighbor(to_connect_1);
            bfs_levels = g.try_get_bfs_tree_levels();
        }

        println!("all should be connected");
        g
    }

    fn try_get_bfs_tree_levels(&self) -> Vec<Option<usize>> {
        let neighbors_list = self.get_neighbors_list();
        let mut levels = vec![None; neighbors_list.len()];
        let mut queue = VecDeque::new();

        levels[0] = Some(0);
        queue.push_back(0);

        while let Some(node) = queue.pop_front() {
            let level = levels[node].unwrap();
            for &neighbor in &neighbors_list[node] {
                if levels[neighbor].is_none() {
                    levels[neighbor] = Some(level + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        levels
    }

    pub fn get_bfs_tree_levels(&self) -> Vec<usize> {
        self.try_get_bfs_tree_levels()
            .iter()
            .map(|n| n.unwrap())
            .collect()
    }

    pub fn root(&self) -> &Node<T> {
        &self.nodes[0]
    }

    pub fn get_neighbors_list(&self) -> Vec<Vec<NodeId>> {
        self.nodes
            .iter()
            .map(|node| node.get_neighbors().clone())
            .collect()
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn init_algorithm(&mut self, init_node_id: NodeId, msg_type: MessageType) {
        self.nodes[init_node_id].receive_message(T::new(
            T::TimeZero,
            None,
            0,
            init_node_id,
            msg_type,
        ));
    }

    pub fn process_round(&mut self) {
        let mut all_messages = vec![];
        for node in &mut self.nodes {
            all_messages.extend(node.process_messages_round());
        }

        self.messages_sent += all_messages.len();

        while let Some(msg) = all_messages.pop() {
            self.nodes[msg.get_receiver()].receive_message(msg);
        }
    }

    pub fn process_message_async(&mut self, msg: T) -> Vec<T> {
        let node = &mut self.nodes[msg.get_receiver()];
        node.receive_message(msg);
        let messages = node.process_messages_round();
        self.messages_sent += messages.len();
        messages
    }

    pub fn get_bfs_data(&self) -> Vec<(usize, usize)> {
        self.nodes.iter().map(|node| node.get_bfs_data()).collect()
    }

    pub fn print_states(&self) {
        for node in &self.nodes {
            println!("{}: {:?}", node.id, node.state.visited);
        }
    }

    pub(crate) fn is_connected(&self) -> bool {
        let mut visited = vec![false; self.nodes.len()];
        for node in &self.nodes {
            for neighbour in &node.neighbors {
                if *neighbour == node.id {
                    panic!("node is neighbour to itself");
                }
                visited[*neighbour] = true;
            }
        }
        visited.iter().all(|v| *v)
    }

    pub fn visited(&self) -> usize {
        self.nodes.iter().filter(|n| n.state.visited).count()
    }
}

impl<T: Message> Display for Graph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}: {:?}", node.id, node.neighbors)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_connected() {
        for n in (5..=100).step_by(5) {
            let g = Graph::<MessageSync>::new(n);
            assert!(g.is_connected());
        }
    }
}
