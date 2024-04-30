use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut graphvec: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Cant open");
    let filereader = BufReader::new(file).lines();
    for line in filereader {  
        let linemsg = line.expect("Error");
        let nodes: Vec<&str> = linemsg.trim().split(' ').collect();
        let node1: usize = nodes[0].parse::<usize>().unwrap();
        let node2: usize = nodes[1].parse::<usize>().unwrap();
        graphvec.push((node1, node2));
    }
    return graphvec;
}
fn create_graph(twitter_data: Vec<(usize, usize)>) -> HashMap<usize, HashSet<usize>> {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (node1, node2) in twitter_data {
        graph.entry(node1).or_insert_with(HashSet::new).insert(node2);
        graph.entry(node2).or_insert_with(HashSet::new).insert(node1);
    }

    graph
}

fn page_rank(graph: &HashMap<usize, HashSet<usize>>, damping: f64, iterations: usize) -> HashMap<usize, f64> {
    let mut ranks: HashMap<usize, f64> = HashMap::new();
    let num_nodes = graph.len() as f64;

    // Initialize each node's rank to 1/N
    for node in graph.keys() {
        ranks.insert(*node, 1.0 / num_nodes);
    }

    for _ in 0..iterations {
        let mut new_ranks: HashMap<usize, f64> = HashMap::new();

        for (node, edges) in graph {
            let outgoing_rank = ranks[node] / edges.len() as f64;

            for edge in edges {
                *new_ranks.entry(*edge).or_insert(0.0) += damping * outgoing_rank;
            }
        }

        // Re-normalize the ranks
        let s: f64 = new_ranks.values().sum();
        for rank in new_ranks.values_mut() {
            *rank += (1.0 - s) / num_nodes;
        }

        ranks = new_ranks;
    }

    ranks
}





fn main() {
    let path = "/Users/rheahalebeed/Desktop/DS210/Final_Project/final_code/src/twitter_combined.txt";
    let twitter_graph = read_file(path);
    let graph = create_graph(twitter_graph);

    let ranks = page_rank(&graph, 0.85, 100);
    for (node, rank) in &ranks {
        println!("Node: {}, Rank: {}", node, rank);
    } 
}