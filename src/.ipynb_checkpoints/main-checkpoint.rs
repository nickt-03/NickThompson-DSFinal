use std::io::{self};
mod graph;

const DATA_FILE: &str = "data/twitter_combined.txt";

fn main() {
    //Step 1: Random sampling of the dataset
    let mut sample_size: Option<usize> = None;

    println!("Due to the size of the dataset, it may take a moment to run the program on the entire dataset. Would you like to randomly sample the nodes from the graph? (yes/no)");

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input = input.trim().to_lowercase();

    //Step 2: How many nodes to sample 
    if input == "yes" {
        println!("How many nodes would you like to randomly sample? (There are 81,306 total Nodes)");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");


        //If invalid number, default to full dataset
        if let Ok(number) = input.trim().parse::<usize>() {
            sample_size = Some(number);
        } else {
            println!("Invalid number. Proceeding with the full dataset.");
        }
    }

    // Step 3: Analyze Graph
    let graph = graph::read_graph(DATA_FILE);
    let total_nodes = graph.len();
    println!("Total nodes in full graph: {}", total_nodes);

    let (sampled_graph, sampled_nodes) = if let Some(size) = sample_size {
        graph::sample_graph(&graph, size)
    } else {
        (graph.clone(), graph.keys().cloned().collect())
    };

    let sampled_nodes_count = sampled_nodes.len();
    println!("Sampled {} nodes", sampled_nodes_count);

    let (num_nodes, num_edges, avg_degree, avg_sep) = graph::analyze_graph(&sampled_graph);
    println!("Sampled graph - Number of nodes: {}", num_nodes);
    println!("Sampled graph - Number of edges: {}", num_edges / 2);
    println!("Sampled graph - Average degree: {:.2}", avg_degree);
    println!("Sampled graph - Average degrees of separation: {:.2}", avg_sep);

    // degree centrality for most influential profiles
    let top_10_centrality = graph::degree_centrality(&sampled_graph);

    // sort for the top 10 most influential 
    let mut degree_vec: Vec<_> = top_10_centrality.iter().collect();
    degree_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nTop 10 Most Influential Twitter Profiles (Highest Degree of Centrality):");
    for (node, degree) in degree_vec.iter().take(10) { 
        println!("Node ID: {} - Degree: {}", node, degree);
    }

    //Step 4: profile suggestions
    println!("Would you like to provide a Node ID for other recommended profiles to follow? (yes/no)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input = input.trim().to_lowercase();

    //ask for node ID to base recommendations on 
    if input == "yes" {
        let mut attempts = 0;
        let mut valid_node_found = false;
        while attempts < 2 && !valid_node_found {
            println!("Please provide a Node ID:");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let node_id: Option<usize> = input.trim().parse().ok();

            if let Some(id) = node_id {
                if sampled_graph.contains_key(&id) {
                    let suggestions = graph::most_shared_neighbors(&sampled_graph, id);

                    //display the top 5 recommendations
                    if suggestions.is_empty() {
                        println!("Node {} has no shared neighbors in the sampled graph.", id);
                    } else {
                        println!("Top 5 Recommended Profiles for Node {}:", id);
                        for (neighbor, shared_count) in suggestions {
                            println!("Node {}: {} shared neighbors", neighbor, shared_count);
                        }
                    }
                    valid_node_found = true;  // Valid input, stop asking
                } else {
                    println!("Invalid Node ID. Please provide a valid numeric Node ID.");
                }
            }
        }
    }
}