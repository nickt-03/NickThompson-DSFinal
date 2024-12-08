pub fn clustering_coefficient(graph: &HashMap<usize, HashSet<usize>>) -> HashMap<usize, f64> {
    let mut coefficients = HashMap::new();

    for (&node, neighbors) in graph {
        let mut triangles = 0;
        let mut possible_triangles = 0;

        //pairs of neighbors
        for &neighbor in neighbors {
            for &other_neighbor in neighbors {
                if neighbor != other_neighbor && graph.get(&neighbor).map_or(false, |n| n.contains(&other_neighbor)) {
                    triangles += 1;
                }
                possible_triangles += 1;
            }
        }

        //clustering coef: number of triangles / possible triangles
        if possible_triangles > 0 {
            coefficients.insert(node, triangles as f64 / possible_triangles as f64);
        } else {
            coefficients.insert(node, 0.0);
        }
    }

    coefficients
}