mod utilities;

use utilities::{AsyncSimulator, Graph, SyncSimulator};

fn build_graph_1() -> Graph {
    let mut g = Graph::new();
    g.add_node(0, vec![1, 2]);
    g.add_node(1, vec![0, 3]);
    g.add_node(2, vec![0, 3]);
    g.add_node(3, vec![1, 2]);
    g
}

fn build_graph_2() -> Graph {
    let mut g = Graph::new();
    g.add_node(0, vec![1, 2, 3]);
    g.add_node(1, vec![0, 4]);
    g.add_node(2, vec![0, 4, 5]);
    g.add_node(3, vec![0]);
    g.add_node(4, vec![1, 2, 5]);
    g.add_node(5, vec![2, 4]);
    g
}

fn build_graph_3() -> Graph {
    let mut g = Graph::new();
    g.add_node(0, vec![1]);
    g.add_node(1, vec![0, 2, 3]);
    g.add_node(2, vec![1, 4]);
    g.add_node(3, vec![1, 5]);
    g.add_node(4, vec![2, 5]);
    g.add_node(5, vec![3, 4]);
    g
}

fn main() {
    println!("Graph 1:");
    let mut sim = SyncSimulator::new(build_graph_1());
    sim.run(0);
    sim.report();

    println!("\nGraph 2:");
    let mut sim = SyncSimulator::new(build_graph_2());
    sim.run(0);
    sim.report();

    println!("\nGraph 3:");
    let mut sim = SyncSimulator::new(build_graph_3());
    sim.run(0);
    sim.report();

    println!("\n\nAsynchronous mode:");
    println!("Graph 1:");
    let mut sim = AsyncSimulator::new(build_graph_1());
    sim.run(0);
    sim.report();
}
