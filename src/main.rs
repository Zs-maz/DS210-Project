use std::collections::HashMap;
use petgraph::graph::NodeIndex;
mod graph;

fn main() {
    // Specify the dataset file path
    let file_path = "../finefoods.txt";

    // Step 1: Load the graph from the dataset
    let graph = graph::read_graph_from_file(file_path);

    // Step 2: Calculate degrees and centralities
    let degrees = graph::calculate_degree(&graph);
    let degree_centrality = graph::calculate_degree_centrality(&graph);

    // Step 3: Display insights about the dataset
    println!("=== Finefoods Dataset Insights ===\n");

    // Display top users/products by degree (connections)
    display_top_nodes("Most Reviewed Products / Active Users (Degree)", &degrees, 10);

    // Display top users/products by degree centrality (normalized connectivity)
    display_top_nodes("Normalized Connectivity (Degree Centrality)", &degree_centrality, 10);

    // Step 4: Calculate and display the average distance between nodes
    let avg_distance = graph::calculate_average_distance(&graph, 1000);
    println!("Average Distance Between Sampled Nodes: {:.2}", avg_distance);
    println!("\n====================================");
}

// Function to display the top nodes by a given metric
fn display_top_nodes(metric: &str, values: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top {} Nodes by {}:", top_count, metric);

    let mut sorted_nodes: Vec<_> = values.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal));

    for (i, (node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.4}", i + 1, node.index(), value);
    }

    println!();
}
