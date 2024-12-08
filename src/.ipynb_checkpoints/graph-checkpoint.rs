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

//Read the graph from the dataset
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

// Randomly sample nodes
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

// Analyze the graph and compute statistics
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

// Compute the average degrees of separation from a given node
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
