use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::{VecDeque};


fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut graphvec: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Cant open");
    let reader = BufReader::new(file).lines();
    for line in reader {  
        let linemsg = line.expect("Error");
        let nodes: Vec<&str> = linemsg.trim().split(' ').collect();
        let n1: usize = nodes[0].parse::<usize>().unwrap();
        let n2: usize = nodes[1].parse::<usize>().unwrap();
        graphvec.push((n1, n2));
    }
    return graphvec;
}
fn create_graph(twitter_data: Vec<(usize, usize)>) -> HashMap<usize, HashSet<usize>> {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (n1, n2) in twitter_data {
        graph.entry(n1).or_insert_with(HashSet::new).insert(n2);
        graph.entry(n2).or_insert_with(HashSet::new).insert(n1);
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
//main analysis

// friends of friends
fn friends_of_friends(graph: &HashMap<usize, HashSet<usize>>, user_id: usize) -> HashSet<usize> {
    let mut friends_of_friends = HashSet::new();

    if let Some(friends) = graph.get(&user_id) {
        for &friend in friends {
            if let Some(friends_of_friend) = graph.get(&friend) {
                for &f_o_f in friends_of_friend {
                    if fof != user_id && !friends.contains(&f_o_f) {
                        friends_of_friends.insert(fof);
                    }
                }
            }
        }
    }

    friends_of_friends
}
//six degrees
fn sixdegrees(graph: &HashMap<usize, HashSet<usize>>, start: usize) -> HashMap<usize, usize> {
    let mut degrees = HashMap::new();
    let mut queue = VecDeque::new();

    degrees.insert(start, 0);
    queue.push_back((start, 0));

    while let Some((node, depth)) = queue.pop_front() {
        if depth < 6 {
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !degrees.contains_key(&neighbor) {
                        queue.push_back((neighbor, depth + 1));
                        degrees.insert(neighbor, depth + 1);
                    }
                }
            }
        }
    }

    degrees
}
// closeness centrality



fn main() {
    let path = "/Users/rheahalebeed/Desktop/DS210/Final_Project/final_code/src/twitter_combined.txt";
    let twitter_graph = read_file(path);
    let graph = create_graph(twitter_graph);

    let ranks = page_rank(&graph, 0.85, 100);
    for (node, rank) in &ranks {
        println!("Node: {}, Rank: {}", node, rank);
    } 

    let user_id = 14804766; // replace with your user ID
    let mutuals = friends_of_friends(&graph, user_id);
    for f_o_f in &mutuals {
        println!("Friend of friend: {}", f_o_f);
    }
    
    let user_id = 14804766;
    let degrees = sixdegrees(&graph, user_id);
    for (user, degree) in &degrees {
        println!("User: {}, Degree: {}", user, degree);
    }
}

