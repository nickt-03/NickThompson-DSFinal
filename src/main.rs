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
}
