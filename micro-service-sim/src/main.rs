mod utilities;

use utilities::graph::Graph;

use crate::utilities::message::{MessageAsync, MessageSync};

fn main() {
    let graph = Graph::<MessageSync>::new(100);
    // println!("{}", graph);

    // zad1
    // let (time, msgs) = utilities::sims::sync_flood_sim(graph.clone());
    // println!("Time: {}, Messages: {}", time, msgs);

    // zad2
    let values = utilities::sims::sync_bfs_sim(graph.clone());
    println!("{:10}|{:10}|{:10}", "node", "parent_id", "level");
    let mut is_bfs_tree = true;
    let bfs_tree_levels = graph.get_bfs_tree_levels();
    for (i, value) in values.into_iter().enumerate() {
        if value.1 != bfs_tree_levels[i] {
            is_bfs_tree = false;
        }
        println!("{:^10}|{:^10}|{:^10}", i, value.0, value.1);
    }
    println!("Is BFS tree: {}", is_bfs_tree);

    // zad 4 sync
    // let time = utilities::sims::sync_flood_sim_with_root_end_detection(graph.clone());
    // println!("Time: {}", time);

    let graph = Graph::<MessageAsync>::new(100);
    // println!("{}", graph);

    // zad3
    // let (time, msgs) = utilities::sims::async_flood_sim(graph.clone());
    // println!("Time: {:.3}, Messages: {}", time, msgs);

    let values = utilities::sims::async_bfs_sim(graph.clone());
    println!("{:10}|{:10}|{:10}", "node", "parent_id", "level");
    let mut is_bfs_tree = true;
    let bfs_tree_levels = graph.get_bfs_tree_levels();
    for (i, value) in values.into_iter().enumerate() {
        if value.1 != bfs_tree_levels[i] {
            is_bfs_tree = false;
        }
        println!("{:^10}|{:^10}|{:^10}", i, value.0, value.1);
    }
    println!("Is BFS tree: {}", is_bfs_tree);

    // zad 4 async
    // let time = utilities::sims::async_flood_sim_with_root_end_detection(graph.clone());
    // println!("Time: {:.3}", time);
}
