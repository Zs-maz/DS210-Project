use std::collections::HashMap;
use petgraph::graph::NodeIndex;
mod graph;

fn main() {
    // Specify the path to the dataset file
    let file_path = "../finefoods.txt.gz";

    // Load the graph from the dataset
    let graph = graph::read_graph_from_file(file_path);

    // Calculate degree and degree centrality
    let degrees = graph::calculate_degree(&graph);
    let degree_centrality = graph::calculate_degree_centrality(&graph);

    // Display top nodes by centrality and degrees
    display_top_nodes("Degree Centrality", &degree_centrality, 10);
    display_top_nodes("Degree", &degrees, 10);

    // Calculate and display average distance
    let avg_distance = graph::calculate_average_distance(&graph, 1000);
    println!("Average Distance: {:.2}", avg_distance);
}

// Function to display top nodes by a given metric
fn display_top_nodes(metric: &str, values: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top {} Nodes by {}:", top_count, metric);

    let mut sorted_nodes: Vec<_> = values.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal));

    for (i, (node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.2}", i + 1, node.index(), value);
    }

    println!();
}
