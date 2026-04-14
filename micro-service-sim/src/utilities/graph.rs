use std::fmt::Display;

use super::{NodeId, P, node::Node};

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        let mut g = Graph {
            nodes: (0..n).map(|i| Node::new(i)).collect(),
        };

        for i in 0..n {
            g.nodes[i].add_neighbors((0..n).filter(|j| *j != i && rand::random_bool(P)).collect());
        }

        todo!(
            "Check if all nodes are neighbors to someone and if not create one random connection from somene to them"
        );
        g
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

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}: {:?}", node.id, node.neighbors)?;
        }

        Ok(())
    }
}
