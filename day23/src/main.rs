use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}", data);

    // parse string input for all the edges
    let (vertices, edges) = data.lines().fold(
        (HashSet::new(), HashMap::new()),
        |(mut vertices, mut edges), line| {
            let (left, right) = line.split_once("-").unwrap();
            vertices.insert(left);
            vertices.insert(right);
            edges.entry(left).or_insert_with(HashSet::new).insert(right);
            edges.entry(right).or_insert_with(HashSet::new).insert(left);
            (vertices, edges)
        },
    );

    let largest_clique = bron_kerbosch(
        &HashSet::new(),
        &mut vertices.iter().cloned().collect(),
        &mut HashSet::new(),
        &edges,
    );

    let mut largest_clique = largest_clique.iter().collect::<Vec<_>>();
    largest_clique.sort();
    let p2 = largest_clique.iter().join(",");
    println!("p2: {}", p2);
    // p2: ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys
}

/*
* It finds the maximal cliques that include all of the vertices in R, some of the vertices in P, and none of the vertices in X
*
    algorithm BronKerbosch1(R, P, X) is
        if P and X are both empty then
            report R as a maximal clique
        for each vertex v in P do
            BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
            P := P \ {v}
            X := X ⋃ {v}
*/
fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    edges: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let mut largest_clique = HashSet::new();
    for vertex in p.clone() {
        let mut r = r.clone();
        r.insert(vertex);

        let neighbors = edges.get(vertex).unwrap();
        let mut new_p = p.intersection(neighbors).cloned().collect();
        let mut new_x = x.intersection(neighbors).cloned().collect();

        let clique = bron_kerbosch(&r, &mut new_p, &mut new_x, edges);
        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }

        p.remove(vertex);
        x.insert(vertex);
    }

    largest_clique
}
