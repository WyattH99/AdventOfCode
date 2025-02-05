use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}", data);

    // parse string input for all the edges
    let edges: Vec<(&str, &str)> = data
        .lines()
        .map(|line| {
            let edge: Vec<&str> = line.split('-').collect();
            (edge[0], edge[1])
        })
        .collect();

    // convert to graph format
    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();
    edges.iter().for_each(|edge| {
        if !conns.contains_key(edge.0) {
            conns.insert(edge.0, HashSet::new());
        }
        if !conns.contains_key(edge.1) {
            conns.insert(edge.1, HashSet::new());
        }
        conns.get_mut(edge.0).map(|val| val.insert(edge.1));
        conns.get_mut(edge.1).map(|val| val.insert(edge.0));
    });

    // iterate through each node connections
    //  iterate through their connections
    //      iterate through those connections
    //          see if any connections are the first node
    let mut sets: HashSet<(&str, &str, &str)> = HashSet::new();
    conns.iter().for_each(|(x, set)| {
        set.iter().for_each(|y| {
            conns.get(y).unwrap().iter().for_each(|z| {
                if x != z && conns.get(z).unwrap().contains(x) {
                    let mut vec: Vec<&str> = vec![x, y, z];
                    vec.sort();
                    sets.insert((vec[0], vec[1], vec[2]));
                }
            });
        });
    });
    //println!("sets: {:?}", sets);

    let mut t: Vec<_> = Vec::new();
    sets.iter().for_each(|tup| {
        if tup.0.starts_with('t') | tup.1.starts_with('t') | tup.2.starts_with('t') {
            t.push(tup);
        }
    });
    //println!("t: {:?}", t);
    println!("len: {}", t.len());
    // len: 1151
}
