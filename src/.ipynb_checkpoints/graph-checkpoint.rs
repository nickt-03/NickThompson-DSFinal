use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::seq::IteratorRandom;

// Utility function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// READ GRAPH. Read the graph from the dataset
pub fn read_graph(file_path: &str) -> HashMap<usize, HashSet<usize>> {
    let mut graph = HashMap::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(edge) = line {
                let nodes: Vec<usize> = edge.split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect();
                if nodes.len() == 2 {
                    graph.entry(nodes[0]).or_insert_with(HashSet::new).insert(nodes[1]);
                    graph.entry(nodes[1]).or_insert_with(HashSet::new).insert(nodes[0]);
                }
            }
        }
    }
    graph
}

// RANDOMLY SAMPLE. Randomly sample nodes and build a subgraph
pub fn sample_graph(
    graph: &HashMap<usize, HashSet<usize>>,
    sample_size: usize,
) -> (HashMap<usize, HashSet<usize>>, HashSet<usize>) {
    let sampled_nodes: HashSet<usize> = graph.keys().cloned()
        .choose_multiple(&mut rand::thread_rng(), sample_size)
        .into_iter()
        .collect();
    let mut sampled_graph = HashMap::new();
    for &node in &sampled_nodes {
        if let Some(neighbors) = graph.get(&node) {
            let filtered_neighbors: HashSet<usize> = neighbors.intersection(&sampled_nodes).cloned().collect();
            if !filtered_neighbors.is_empty() {
                sampled_graph.insert(node, filtered_neighbors);
            }
        }
    }
    (sampled_graph, sampled_nodes)
}

// ANALYZE. Analyze the graph and compute statistics
pub fn analyze_graph(
    graph: &HashMap<usize, HashSet<usize>>,
) -> (usize, usize, f64, f64) {
    let num_nodes = graph.len();
    let num_edges: usize = graph.values().map(|neighbors| neighbors.len()).sum();
    let avg_degree = num_edges as f64 / num_nodes as f64;

    // Compute average degrees of separation
    let random_node = *graph.keys().next().unwrap();
    let avg_degrees_of_separation = compute_avg_degrees_of_separation(graph, random_node);

    (num_nodes, num_edges, avg_degree, avg_degrees_of_separation)
}

// AVERAGE DEGREES OF SEPARATION. Compute the average degrees of separation from a given node
pub fn compute_avg_degrees_of_separation(
    graph: &HashMap<usize, HashSet<usize>>,
    start: usize,
) -> f64 {
    let mut visited = HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    let mut total_distance = 0;
    let mut num_reachable_nodes = 0;

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current, distance)) = queue.pop_front() {
        total_distance += distance;
        num_reachable_nodes += 1;

        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
    }

    total_distance as f64 / num_reachable_nodes as f64
}

// DEGREE CENTRALITY. Calculate degree centrality for most inflential profiles - most neighbors

pub fn degree_centrality(graph: &HashMap<usize, HashSet<usize>>) -> HashMap<usize, usize> {
    let mut centrality = HashMap::new();

    for (&node, neighbors) in graph {
        // Degree centrality is number of neighbors
        centrality.insert(node, neighbors.len());
    }

    centrality
}

// TOP 5 SHARED NEIGHBORS. Function to get the top 5 most shared neighbors for a given node. Useful for suggesting profiles a user might want to follow.
pub fn most_shared_neighbors(
    graph: &HashMap<usize, HashSet<usize>>,
    selected_node: usize,
) -> Vec<(usize, usize)> {
    let binding = HashSet::new();
    let selected_neighbors = graph
        .get(&selected_node)
        .unwrap_or(&binding);

    let mut shared_counts: Vec<(usize, usize)> = graph
        .iter()
        .filter_map(|(&node, neighbors)| {
            if node != selected_node {
                let shared_neighbors = selected_neighbors
                    .intersection(neighbors)
                    .count();
                if shared_neighbors > 0 {
                    Some((node, shared_neighbors))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    shared_counts.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by shared neighbor count, descending
    shared_counts.truncate(5); // Keep top 5

    shared_counts
}
