use std::collections::{HashMap, HashSet};
use crate::graph::{compute_avg_degrees_of_separation, sample_graph, degree_centrality};

// Create a simple test graph
fn create_test_graph() -> HashMap<usize, HashSet<usize>> {
    let mut graph = HashMap::new();
    graph.insert(1, HashSet::from([2, 3]));
    graph.insert(2, HashSet::from([1, 3]));
    graph.insert(3, HashSet::from([1, 2, 4]));
    graph.insert(4, HashSet::from([3]));
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    // Graph sampling 
    #[test]
    fn test_sample_graph() {
        let graph = create_test_graph();
        let (sampled_graph, sampled_nodes) = sample_graph(&graph, 2);

        // Check that two nodes are sampled
        assert_eq!(sampled_nodes.len(), 2);
        // Check that the sampled graph contains only the sampled nodes
        assert!(sampled_graph.len() <= 2);
        for node in &sampled_nodes {
            assert!(graph.contains_key(node));
        }
    }

    // Degree centrality 
    #[test]
    fn test_degree_centrality() {
        let graph = create_test_graph();
        let centrality = degree_centrality(&graph);

        // Verify the degree centrality of specific nodes
        assert_eq!(centrality[&1], 2);
        assert_eq!(centrality[&3], 3);
    }

    // Average degrees of separation
    #[test]
    fn test_compute_avg_degrees_of_separation() {
        let graph = create_test_graph();
        let avg_separation = compute_avg_degrees_of_separation(&graph, 1);

        // Check that the average degrees of separation is within expected range
        assert!(avg_separation >= 1.0 && avg_separation <= 2.0);
    }
}