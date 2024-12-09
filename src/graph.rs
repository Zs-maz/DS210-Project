use petgraph::graph::{Graph, NodeIndex};
use flate2::read::GzDecoder;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::IteratorRandom;

// Function to read and parse the dataset file into a graph
pub fn read_graph_from_file(file_path: &str) -> Graph<String, ()> {
    let mut graph = Graph::<String, ()>::with_capacity(0, 0);
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut current_user: Option<String> = None;
    let mut current_product: Option<String> = None;

    // Open the compressed GZIP file
    let file = File::open(file_path).expect("Failed to open dataset file");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // A blank line marks the end of a record
        if line.trim().is_empty() {
            if let (Some(user), Some(product)) = (current_user.take(), current_product.take()) {
                let user_node = *node_map.entry(user.clone()).or_insert_with(|| graph.add_node(user));
                let product_node = *node_map.entry(product.clone()).or_insert_with(|| graph.add_node(product));
                graph.add_edge(user_node, product_node, ());
            }
            continue;
        }

        // Parse key-value pairs
        if let Some((key, value)) = line.split_once(": ") {
            match key.trim() {
                "review/userId" => current_user = Some(value.trim().to_string()),
                "product/productId" => current_product = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }

    graph
}

// Function to calculate the degree of each node in the graph
pub fn calculate_degree(graph: &Graph<String, ()>) -> HashMap<NodeIndex, f64> {
    let mut degrees = HashMap::new();
    for node in graph.node_indices() {
        degrees.insert(node, graph.neighbors(node).count() as f64);
    }
    degrees
}

// Function to calculate the degree centrality of each node
pub fn calculate_degree_centrality(graph: &Graph<String, ()>) -> HashMap<NodeIndex, f64> {
    let max_possible_degree = (graph.node_count() - 1) as f64;
    let mut degree_centrality = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count() as f64;
        degree_centrality.insert(node, degree / max_possible_degree);
    }
    degree_centrality
}

// Function to calculate the average distance between nodes using BFS sampling
pub fn calculate_average_distance(graph: &Graph<String, ()>, size: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let nodes: Vec<_> = graph.node_indices().collect();
    let subset: Vec<_> = nodes.iter().choose_multiple(&mut rng, size).into_iter().cloned().collect();

    let mut total_distance = 0;
    let mut total_pairs = 0;

    for &start_node in &subset {
        let distances = bfs_distances(graph, start_node);
        for (_end_node, distance) in distances {
            total_distance += distance;
            total_pairs += 1;
        }
    }

    if total_pairs > 0 {
        total_distance as f64 / total_pairs as f64
    } else {
        0.0
    }
}

// Helper function: Perform BFS to calculate distances from a given source node
pub fn bfs_distances(graph: &Graph<String, ()>, source: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((source, 0));
    distances.insert(source, 0);

    while let Some((current_node, current_distance)) = queue.pop_front() {
        for neighbor in graph.neighbors(current_node) {
            if !distances.contains_key(&neighbor) {
                let new_distance = current_distance + 1;
                queue.push_back((neighbor, new_distance));
                distances.insert(neighbor, new_distance);
            }
        }
    }
    distances
}

// Function to display top nodes by a given metric
pub fn display_top_nodes(metric: &str, values: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top {} Nodes by {}:", top_count, metric);

    let mut sorted_nodes: Vec<_> = values.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal));

    for (i, (node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.2}", i + 1, node.index(), value);
    }

    println!();
}
