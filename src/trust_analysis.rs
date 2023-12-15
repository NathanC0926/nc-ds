use crate::{trust_analysis};
use super::csv_parsing::Record;

// Function to compute the average trust score for the network
pub fn calculate_network_average_trust(records: &[Record]) -> f64 {
    let total_trust: i32 = records.iter().map(|record| record.rating).sum();
    total_trust as f64 / records.len() as f64
}

// Function to compute the average trust score for a set of nodes
pub fn analyze_trust_scores(records: &[Record], nodes_to_analyze: &[i32]) -> f64 {
    let mut total_average_score: f64 = 0.0;
    let mut count = 0;

    for &node in nodes_to_analyze {
        let filtered_records: Vec<&Record> = records.iter()
            .filter(|record| record.target == node)
            .collect();

        let sum_ratings: i32 = filtered_records.iter()
            .map(|record| record.rating)
            .sum();

        let ratings_count = filtered_records.len() as f64;

        if ratings_count > 0.0 {
            let average_score = sum_ratings as f64 / ratings_count;
            total_average_score += average_score;
            count += 1;
        }
    }

    total_average_score / count as f64
}

// Function for printing trust scores, for the sake of not
pub fn calculate_and_print_group_trust_scores(records: &[Record], nodes: &[i32], group_name: &str) {
    let average_trust = trust_analysis::analyze_trust_scores(records, nodes);
    println!("Average Trust Score for {} group: {:.4}", group_name, average_trust);
}


mod tests {
    use super::*;
    #[test]
fn test_calculate_network_average_trust() {
    let records = vec![
        Record { source: 1, target: 2, rating: 5 },
        Record { source: 2, target: 3, rating: 3 },
        Record { source: 3, target: 4, rating: 4 },
        Record { source: 4, target: 1, rating: 2 },
        Record { source: 2, target: 4, rating: 1 },
    ];

    let average_trust = trust_analysis::calculate_network_average_trust(&records);
    let expected_average = 3.0; // (5+3+4+2+1)/5 = 3

    assert_eq!(average_trust, expected_average);
}
    #[test]
    fn test_analyze_trust_scores() {
        let records = vec![
            Record { source: 1, target: 2, rating: 5 },
            Record { source: 2, target: 3, rating: 3 },
            Record { source: 3, target: 2, rating: 4 },
            Record { source: 4, target: 1, rating: 2 },
            Record { source: 2, target: 4, rating: 1 },
        ];

        let nodes_to_analyze = vec![1,2]; 
        let average_score = analyze_trust_scores(&records, &nodes_to_analyze);

        let expected_average = 3.25 ; // node 1: 2, node 2: 5+4/2 = 4.5, (4.5+2)/2 = 3.5

        assert_eq!(average_score, expected_average);
    }
}
