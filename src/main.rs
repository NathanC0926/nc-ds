mod csv_parsing;
mod graph_analysis;
mod trust_analysis;
use rustworkx_core;
use rustworkx_core::petgraph;
use serde::Deserialize;
use std::error::Error;
use petgraph::prelude::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "C:/Users/natha/Downloads/soc-sign-bitcoinalpha.csv";
    let records = csv_parsing::read_csv(file_path)?;
    let graph = graph_analysis::construct_graph(&records);

    graph_analysis::calculate_and_print_pagerank(&records);
    graph_analysis::calculate_and_print_betweenness_centrality(&graph);
    graph_analysis::calculate_and_print_closeness_centrality(&graph);



    let (in_degree_sorted, out_degree_sorted) = graph_analysis::calculate_degree_centrality(&graph);

    // Print the top 38 nodes by unweighted in-degree
    println!("Top 38 Nodes by Unweighted In-Degree:");
    for (node, in_degree, out_degree, mean_in_weight, mean_out_weight) in in_degree_sorted.iter().take(38) {
        println!("Node {} - Unweighted In-Degree: {}, Unweighted Out-Degree: {}, Mean Weighted In-Degree: {:.2}, Mean Weighted Out-Degree: {:.2}", node, in_degree, out_degree, mean_in_weight, mean_out_weight);
    }

    // Print the top 38 nodes by unweighted out-degree
    println!("\nTop 38 Nodes by Unweighted Out-Degree:");
    for (node, in_degree, out_degree, mean_in_weight, mean_out_weight) in out_degree_sorted.iter().take(38) {
        println!("Node {} - Unweighted In-Degree: {}, Unweighted Out-Degree: {}, Mean Weighted In-Degree: {:.2}, Mean Weighted Out-Degree: {:.2}", node, in_degree, out_degree, mean_in_weight, mean_out_weight);
    }

    let groups = vec![
        (vec![1, 8, 30], "top_in_4_or_5"),
        (vec![2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 22, 25, 26, 33, 38, 40, 43, 46, 58, 177], "top_in_3"),
        (vec![19, 21, 24, 27, 35, 42, 49, 53, 70, 79, 95, 129, 133, 135, 136, 139, 149, 349, 535, 540, 547, 564, 612, 624, 659, 681, 688, 690, 711, 715, 798, 876, 7564], "top_in_2"),
        (vec![23, 28, 29, 36, 37, 50, 57, 61, 69, 72, 85, 145, 174, 269, 276, 284, 373, 541, 542, 555, 562, 563, 579, 589, 617, 682, 816, 884, 1442, 2336, 5342, 7603], "top_in_1"),
    ];

    let network_average_trust = trust_analysis::calculate_network_average_trust(&records);
    println!("Network Average Trust Score: {:.4}", network_average_trust);

    // Iterate over each group and print trust scores
    for (nodes, group_name) in groups {
        trust_analysis::calculate_and_print_group_trust_scores(&records, &nodes, group_name);
    }

    Ok(())
}

