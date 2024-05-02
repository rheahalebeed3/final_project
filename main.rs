use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::{VecDeque};

fn read_file(path: &str) -> Vec<(usize, usize)> {
    //define graph vector
    let mut graphvector: Vec<(usize, usize)> = Vec::new();
    //file and reader
    let file = File::open(path).expect("Cant open");
    let reader = BufReader::new(file).lines();
    //each line read
    for line in reader {  
        let linemsg = line.expect("Error");
        let nodes: Vec<&str> = linemsg.trim().split(' ').collect();
        let n1: usize = nodes[0].parse::<usize>().unwrap();
        let n2: usize = nodes[1].parse::<usize>().unwrap();
        graphvector.push((n1, n2));
    }
    return graphvector;
}
//create graph
fn create_graph(twitter_data: Vec<(usize, usize)>) -> HashMap<usize, HashSet<usize>> {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (n1, n2) in twitter_data {
        graph.entry(n1).or_insert_with(HashSet::new).insert(n2);
        graph.entry(n2).or_insert_with(HashSet::new).insert(n1);
    }

    graph
}
//pagerank
fn page_rank(graph: &HashMap<usize, HashSet<usize>>, damping: f64, iterations: usize) -> HashMap<usize, f64> {
    //defining variables
    let mut ranks: HashMap<usize, f64> = HashMap::new();
    let num_nodes = graph.len() as f64;
// for each node insert rank
    for node in graph.keys() {
        ranks.insert(*node, 1.0 / num_nodes);
    }
    //for each iteration new tanks into a new hashmap
    for _ in 0..iterations {
        let mut assign_ranks: HashMap<usize, f64> = HashMap::new();
        //for each node and edge calculate rank
        for (node, edges) in graph {
            let new_ranks = ranks[node] / edges.len() as f64;
        
            for edge in edges {
                *assign_ranks.entry(*edge).or_insert(0.0) += damping * new_ranks;
            }
        }
        
        let s: f64 = assign_ranks.values().sum();
        for rank in assign_ranks.values_mut() {
            *rank += (1.0 - s) / num_nodes;
        }

        ranks = assign_ranks;
    }

    ranks
}
//main analysis

// friends of friends
fn friends_of_friends(graph: &HashMap<usize, HashSet<usize>>, user_id: usize) -> HashSet<usize> {
    //defining variable
    let mut friends_of_friends = HashSet::new();
    //user ids
    if let Some(friends) = graph.get(&user_id) {
        //for each friend find friends of frienxs
        for &friend in friends {
            if let Some(friends_of_friend) = graph.get(&friend) {
                for &f_o_f in friends_of_friend {
                    if f_o_f != user_id && !friends.contains(&f_o_f) {
                        friends_of_friends.insert(f_o_f);
                    }
                }
            }
        }
    }

    friends_of_friends
}
//six degrees
fn sixdegrees(graph: &HashMap<usize, HashSet<usize>>, start: usize) -> HashMap<usize, usize> {
    //defining variable
    let mut degrees = HashMap::new();
    let mut queue = VecDeque::new();
    //degree and queue 
    degrees.insert(start, 0);
    queue.push_back((start, 0));
    //depth conditions for degree calculuation
    while let Some((node, depth)) = queue.pop_front() {
        if depth < 6 {
            if let Some(neighbor_nodes) = graph.get(&node) {
                for &neighbor in neighbor_nodes {
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
// bfs
fn bfs(graph: &HashMap<usize, HashSet<usize>>, start: usize) {
    //defining variable
    let mut visit_history: HashSet<usize> = HashSet::new();
    let mut line = VecDeque::new();

    visit_history.insert(start);
    line.push_back(start);
//while on node pop and print
    while let Some(node) = line.pop_front() {
        println!("{}", node);
        //neighbor conditons
        if let Some(neighbor_nodes) = graph.get(&node) {
            for &neighbor in neighbor_nodes {
                if !visit_history.contains(&neighbor) {
                    visit_history.insert(neighbor);
                    line.push_back(neighbor);
                }
            }
        }
    }
}

fn main() {
    //path read and graph creation
    let path = "/Users/rheahalebeed/Desktop/DS210/Final_Project/final_code/src/twitter_combined.txt";
    let twitter_graph = read_file(path);
    let graph = create_graph(twitter_graph);
//pagerank
    let ranks = page_rank(&graph, 0.85, 100);
    println!("Top 10 nodes by PageRank:");
    for (node, rank) in ranks.iter().take(10) {
        println!("Node: {}, Rank: {}", node, rank);
    } 
//friends of friends  
    let user_id = 14804766; 
    let mutuals = friends_of_friends(&graph, user_id);
    println!("Top 10 friends of friends:");
    for f_o_f in mutuals.iter().take(10) {
        println!("Friend of friend: {}", f_o_f);
    }
    //six degrees
    println!("Six Degrees");
    let user_id = 14804766;
    let degrees = sixdegrees(&graph, user_id);
    for (user, degree) in degrees.iter().take(10) {
        println!("User: {}, Degree: {}", user, degree);
    }
    //bfs
    bfs(&graph,0);
    
}
