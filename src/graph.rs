use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_graph_from_file(file_path: &str) -> Graph<String, ()> {
    let mut graph = Graph::<String, ()>::with_capacity(0, 0);
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut current_user: Option<String> = None;
    let mut current_product: Option<String> = None;

    let file = File::open(file_path).expect("Failed to open dataset file");
    let reader = BufReader::new(file);

    for line in reader.split(b'\n') {
        match line {
            Ok(line) => {
                let line = String::from_utf8_lossy(&line).trim().to_string();

                // A blank line marks the end of a record
                if line.is_empty() {
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
            Err(e) => {
                eprintln!("Skipping invalid line: {:?}", e);
            }
        }
    }

    graph
}

/// Calculates the degree (number of connections) for each node in the graph.
pub fn calculate_degree(graph: &Graph<String, ()>) -> HashMap<NodeIndex, f64> {
    let mut degrees = HashMap::new();
    for node in graph.node_indices() {
        degrees.insert(node, graph.neighbors(node).count() as f64);
    }
    degrees
}

/// Calculates the degree centrality (normalized degree) for each node.
pub fn calculate_degree_centrality(graph: &Graph<String, ()>) -> HashMap<NodeIndex, f64> {
    let mut degree_centrality = HashMap::new();

    if graph.node_count() > 1 {
        let max_possible_degree = (graph.node_count() - 1) as f64;

        for node in graph.node_indices() {
            let degree = graph.neighbors(node).count() as f64;
            degree_centrality.insert(node, degree / max_possible_degree);
        }
    } else {
        for node in graph.node_indices() {
            degree_centrality.insert(node, 0.0); // For empty or single-node graph
        }
    }

    degree_centrality
}

/// Calculates the average distance between sampled nodes in the graph.
pub fn calculate_average_distance(graph: &Graph<String, ()>, sample_size: usize) -> f64 {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    let nodes: Vec<_> = graph.node_indices().collect();
    let sampled_nodes: Vec<_> = nodes.choose_multiple(&mut rng, sample_size).cloned().collect();

    let mut total_distance = 0;
    let mut total_pairs = 0;

    for &start_node in &sampled_nodes {
        let distances = bfs_distances(graph, start_node);
        for (_, distance) in distances {
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

/// Performs Breadth-First Search (BFS) to calculate distances from a start node.
fn bfs_distances(graph: &Graph<String, ()>, start: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((current_node, current_distance)) = queue.pop_front() {
        for neighbor in graph.neighbors(current_node) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, current_distance + 1);
                queue.push_back((neighbor, current_distance + 1));
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_degree() {
        let mut graph = Graph::<String, ()>::new();
        let node1 = graph.add_node("User1".to_string());
        let node2 = graph.add_node("Product1".to_string());
        graph.add_edge(node1, node2, ());
        let degrees = calculate_degree(&graph);
        assert_eq!(degrees[&node1], 1.0);
        assert_eq!(degrees[&node2], 1.0);
    }

    #[test]
    fn test_calculate_degree_centrality() {
        let mut graph = Graph::<String, ()>::new();
        let node1 = graph.add_node("User1".to_string());
        let node2 = graph.add_node("Product1".to_string());
        graph.add_edge(node1, node2, ());
        let centrality = calculate_degree_centrality(&graph);
        assert!(centrality[&node1] > 0.0);
        assert!(centrality[&node2] > 0.0);
    }

    #[test]
    fn test_bfs_distances() {
        let mut graph = Graph::<String, ()>::new();
        let node1 = graph.add_node("User1".to_string());
        let node2 = graph.add_node("Product1".to_string());
        graph.add_edge(node1, node2, ());
        let distances = bfs_distances(&graph, node1);
        assert_eq!(distances[&node2], 1);
    }
}
