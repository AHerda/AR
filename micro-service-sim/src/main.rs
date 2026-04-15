mod utilities;

use utilities::graph::Graph;

use crate::utilities::message::{MessageAsync, MessageSync};

fn main() {
    // let graph = Graph::<MessageSync>::new(100);
    // println!("{}", graph);

    // zad1
    // let (time, msgs) = utilities::sims::sync_flood_sim(graph.clone());
    // println!("Time: {}, Messages: {}", time, msgs);

    // zad2
    // let values = utilities::sims::sync_bfs_sim(graph.clone());
    // println!("{:10}|{:10}|{:10}", "node", "parent_id", "level");
    // for (i, value) in values.into_iter().enumerate() {
    //     println!("{:^10}|{:^10}|{:^10}", i, value.0, value.1);
    // }

    let graph = Graph::<MessageAsync>::new(100);
    // println!("{}", graph);

    // zad3
    // let (time, msgs) = utilities::sims::async_flood_sim(graph.clone());
    // println!("Time: {}, Messages: {}", time, msgs);

    let values = utilities::sims::async_bfs_sim(graph.clone());
    println!("{:10}|{:10}|{:10}", "node", "parent_id", "level");
    for (i, value) in values.into_iter().enumerate() {
        println!("{:^10}|{:^10}|{:^10}", i, value.0, value.1);
    }
}
