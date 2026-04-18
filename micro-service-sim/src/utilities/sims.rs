use std::collections::{BinaryHeap, VecDeque};

use crate::utilities::NodeId;

use super::{
    graph::Graph,
    message::{Message, MessageAsync, MessageSync, MessageType},
};

/// @returns: (time, #sent msgs)
pub fn sync_flood_sim(mut graph: Graph<MessageSync>) -> (usize, usize) {
    let mut time = 0;
    graph.init_algorithm(0, MessageType::Flood);

    while graph.visited() != graph.size() {
        graph.process_round();
        time += 1;

        if time > 1000 {
            eprintln!("Graph not fully connected");
            break;
        }
    }

    (time, graph.messages_sent)
}

/// @returns: Vec<parent_id, level>
pub fn sync_bfs_sim(mut graph: Graph<MessageSync>) -> Vec<(NodeId, usize)> {
    let mut time = 0;
    graph.init_algorithm(0, MessageType::Bfs);

    while graph.visited() != graph.size() {
        graph.process_round();
        time += 1;

        if time > 1000 {
            eprintln!("Graph not fully connected");
            break;
        }
    }

    graph.get_bfs_data()
}

/// @returns: (time, #sent msgs)
pub fn sync_flood_sim_with_root_end_detection(mut graph: Graph<MessageSync>) -> usize {
    let mut time = 0;
    graph.init_algorithm(0, MessageType::Flood);

    while !graph.root().got_all_acks() {
        graph.process_round();
        time += 1;

        if time > 1000 {
            eprintln!("Graph not fully connected");
            break;
        }
    }

    time
}

/// @returns: (time, #sent msgs)
pub fn async_flood_sim(mut graph: Graph<MessageAsync>) -> (f64, usize) {
    let mut time = 0.0;
    let mut messages = BinaryHeap::new();
    messages.push(MessageAsync::new(
        MessageAsync::TimeZero,
        None,
        0,
        0,
        MessageType::Flood,
    ));

    while graph.visited() != graph.size() && !messages.is_empty() {
        let msg = messages.pop().unwrap();
        time = msg.time;
        messages.extend(graph.process_message_async(msg));
    }

    if graph.visited() != graph.size() {
        eprintln!("Graph not fully connected");
    }

    (time, graph.messages_sent)
}

/// @returns: Vec<(parent_id, level)>
pub fn async_bfs_sim(mut graph: Graph<MessageAsync>) -> Vec<(NodeId, usize)> {
    let mut messages = BinaryHeap::new();
    messages.push(MessageAsync::new(
        MessageAsync::TimeZero,
        None,
        0,
        0,
        MessageType::Bfs,
    ));

    while graph.visited() != graph.size() && !messages.is_empty() {
        let msg = messages.pop().unwrap();
        messages.extend(graph.process_message_async(msg));
    }

    if graph.visited() != graph.size() {
        eprintln!("Graph not fully connected");
    }

    graph.get_bfs_data()
}

/// @returns: (time, #sent msgs)
pub fn async_flood_sim_with_root_end_detection(mut graph: Graph<MessageAsync>) -> f64 {
    let mut time = 0.0;
    let mut messages = BinaryHeap::new();
    messages.push(MessageAsync::new(
        MessageAsync::TimeZero,
        None,
        0,
        0,
        MessageType::Flood,
    ));

    while !graph.root().got_all_acks() {
        let msg = messages.pop().unwrap();
        time = msg.time;
        messages.extend(graph.process_message_async(msg));
    }

    if graph.visited() != graph.size() {
        eprintln!("Graph not fully visited?!?!");
    }

    time
}
