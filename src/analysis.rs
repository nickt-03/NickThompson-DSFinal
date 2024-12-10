//Some code for extra analysis of the graph. Excluded from the actual output of the program, as the nature of the analysis takes a long time to run and can provide a confusing output. 
//The clustering coefficients can provide us with clustered communities based on shared neighbors, and thus, shared interests. An example might be a cluster of accounts relating to Boston Sports. This provides us with the extent to which social circles exist on twitter under communities of shared interests.

//The graph diameter can provide us with a measure of how closely connected the nodes in the graph are. This shows the connectedness of Twitter as a social network, as well as the extent to which different communities vary within the network's entirety. Along with the clustering coefficients, these can be used to test the global connectivity of Twitter. 

use std::collections::{HashMap, HashSet};

#[allow(dead_code)] 
pub fn clustering_coefficient(graph: &HashMap<usize, HashSet<usize>>) -> HashMap<usize, f64> {
    let mut coefficients = HashMap::new();

    for (&node, neighbors) in graph {
        let mut triangles = 0;
        let mut possible_triangles = 0;

        // Check pairs of neighbors for possible triangles
        for &neighbor in neighbors {
            for &other_neighbor in neighbors {
                if neighbor != other_neighbor && graph.get(&neighbor).map_or(false, |n| n.contains(&other_neighbor)) {
                    triangles += 1;
                }
                possible_triangles += 1;
            }
        }

        // Clustering coefficient: number of triangles / possible triangles
        if possible_triangles > 0 {
            coefficients.insert(node, triangles as f64 / possible_triangles as f64);
        } else {
            coefficients.insert(node, 0.0);
        }
    }

    coefficients
}

#[allow(dead_code)] 
pub fn graph_diameter(graph: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut max_distance = 0;

    for &start in graph.keys() {
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut distances = HashMap::new();

        visited.insert(start);
        queue.push_back(start);
        distances.insert(start, 0);

        while let Some(current) = queue.pop_front() {
            let current_distance = *distances.get(&current).unwrap();
            for &neighbor in graph.get(&current).unwrap() {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }

        // Get the farthest node distance from start
        if let Some(&max) = distances.values().max() {
            max_distance = max_distance.max(max);
        }
    }

    max_distance
}
