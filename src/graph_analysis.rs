use petgraph::graph::DiGraph;
use std::collections::HashMap;
use petgraph::prelude::*;
use rustworkx_core::centrality::{betweenness_centrality, closeness_centrality};
use simple_pagerank::Pagerank;
use super::csv_parsing::Record; // Import Record struct from csv_parsing module

// Function to calculate and return the degree centrality of nodes in a directed graph
// Outputs (node, unweighted in-degree, unweighted out-degree, mean weighted in-degree, mean weighted out-degree)
pub fn calculate_degree_centrality(graph: &DiGraph<i32, i32>) -> (Vec<(i32, usize, usize, f32, f32)>, Vec<(i32, usize, usize, f32, f32)>) {
    let mut node_details = Vec::new();

    for node in graph.node_indices() {
        // Calculate unweighted in-degree and out-degree
        let in_degree = graph.edges_directed(node, Incoming).count();
        let out_degree = graph.edges_directed(node, Outgoing).count();

        // Calculate weighted in-degree and out-degree (sum of weights)
        let total_in_weight: i32 = graph.edges_directed(node, Incoming)
            .map(|edge| *edge.weight())
            .sum();
        let total_out_weight: i32 = graph.edges_directed(node, Outgoing)
            .map(|edge| *edge.weight())
            .sum();

        // Calculate mean weighted in-degree and out-degree
        let mean_in_weight = if in_degree > 0 { total_in_weight as f32 / in_degree as f32 } else { 0.0 };
        let mean_out_weight = if out_degree > 0 { total_out_weight as f32 / out_degree as f32 } else { 0.0 };

        // Collect details of each node
        node_details.push((graph[node], in_degree, out_degree, mean_in_weight, mean_out_weight));
    }

    // Sort by unweighted in-degree
    node_details.sort_by(|a, b| b.1.cmp(&a.1));
    let in_degree_sorted = node_details.clone();

    // Sort by unweighted out-degree
    node_details.sort_by(|a, b| b.2.cmp(&a.2));
    let out_degree_sorted = node_details.clone();

    // Return the sorted lists of nodes by in-degree and out-degree
    (in_degree_sorted, out_degree_sorted)
}

// simple_pagerank does not support the pet graph data structures.
// So we have to recreate a directed graph with the method that it supports
pub fn construct_graph(records: &[Record]) -> DiGraph<i32, i32> {

    let mut graph = DiGraph::<i32, i32>::new();
    let mut node_map: HashMap<i32, NodeIndex> = HashMap::new();

    for record in records {
        let source_index = *node_map.entry(record.source).or_insert_with(|| graph.add_node(record.source));
        let target_index = *node_map.entry(record.target).or_insert_with(|| graph.add_node(record.target));
        graph.add_edge(source_index, target_index, record.rating);
    }

    graph
}

// https://crates.io/crates/simple-pagerank
// Very simple implementation of the PageRank algorithm.
pub fn calculate_and_print_pagerank(records: &[Record]) {
    let mut pr = Pagerank::new();
    for record in records {
        pr.add_edge(record.source.to_string(), record.target.to_string());
    }
    pr.calculate();
    println!("Top 38 Nodes by PageRank Score:");
    pr.nodes()
        .iter()
        .take(38)
        .map(|(node, score)| println!("page {} with score {}", node, score))
        .for_each(drop);
}

//https://docs.rs/rustworkx-core/latest/rustworkx_core/centrality/fn.betweenness_centrality.html
// Compute the betweenness centrality of all nodes in a graph.
pub fn calculate_and_print_betweenness_centrality(graph: &DiGraph<i32, i32>) {
    let betweenness_scores = betweenness_centrality(graph, true, true, 50);
    let mut centrality_scores: Vec<(NodeIndex, f64)> = graph.node_indices()
        .zip(betweenness_scores.into_iter())
        .filter_map(|(node, score)| Some((node, score?)))
        .collect();
    centrality_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("Top 38 Nodes by Betweenness Centrality:");
    for (node, score) in centrality_scores.iter().take(38) {
        println!("{:?} - Betweenness Centrality: {:.4}", node, score);
    }
}

pub fn calculate_and_print_closeness_centrality(graph: &DiGraph<i32, i32>) {
    let closeness_scores = closeness_centrality(graph, true);
    let mut centrality_cscores: Vec<(NodeIndex, f64)> = graph.node_indices()
        .zip(closeness_scores.into_iter())
        .filter_map(|(node, score)| Some((node, score?)))
        .collect();
    centrality_cscores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("Top 38 Nodes by Closeness Centrality:");
    for (node, score) in centrality_cscores.iter().take(38) {
        println!("{:?} - Closeness Centrality: {:.4}", node, score);
    }
}


mod tests {
    use super::*;
    use petgraph::graph::{DiGraph};

    #[test]
    fn test_calculate_degree_centrality() {
        // Create a simple directed graph for testing
        let mut graph = DiGraph::new();

        // Add nodes and edges to the graph
        let node_a = graph.add_node(1);
        let node_b = graph.add_node(2);
        let node_c = graph.add_node(3);

        graph.add_edge(node_a, node_b, 1);
        graph.add_edge(node_b, node_c, 2);
        graph.add_edge(node_c, node_a, 3);

        // Call your function to calculate degree centrality
        let (in_degree_sorted, out_degree_sorted) = calculate_degree_centrality(&graph);

        // Perform assertions based on your expected results
        // Replace these with your expected results
        assert_eq!(in_degree_sorted, vec![
            (1, 1, 1, 3.0, 1.0),
            (2, 1, 1, 1.0, 2.0),
            (3, 1, 1, 2.0, 3.0)
        ]);
    }

    #[test]
    fn test_closeness_centrality() {
        let g = petgraph::graph::UnGraph::<i32, ()>::from_edges(&[
            (0, 4), (1, 2), (2, 3), (3, 4), (1, 4)
        ]);
        // Calculate the betweenness centrality
        let output = betweenness_centrality(&g, true, true, 200);
        assert_eq!(
            vec![Some(0.4), Some(0.5), Some(0.45), Some(0.5), Some(0.75)],
            output
        );
    }

    #[test]
    fn test_betweenness_centrality() {
        // Calculate the closeness centrality of Graph
        let g = petgraph::graph::UnGraph::<i32, ()>::from_edges(&[
            (0, 4), (1, 2), (2, 3), (3, 4), (1, 4)
        ]);
        let output = closeness_centrality(&g, true);
        assert_eq!(
            vec![Some(1. / 2.), Some(2. / 3.), Some(4. / 7.), Some(2. / 3.), Some(4. / 5.)],
            output
        );

    // Calculate the closeness centrality of DiGraph
        let dg = petgraph::graph::DiGraph::<i32, ()>::from_edges(&[
            (0, 4), (1, 2), (2, 3), (3, 4), (1, 4)
        ]);
        let output = closeness_centrality(&dg, true);
        assert_eq!(
            vec![Some(0.), Some(0.), Some(1. / 4.), Some(1. / 3.), Some(4. / 5.)],
            output
        );
    }
}
