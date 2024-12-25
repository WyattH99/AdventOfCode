use cgmath::Vector2;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("data: {:?}", data);

    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let row_len = grid.len() as isize;
    let col_len = grid[0].len() as isize;

    // Vec of positions for everything thats not a '.'
    let antennas: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_num, ch)| (Vector2::new(row_num as isize, col_num as isize), ch))
        })
        .filter(|tuple| *tuple.1 != '.')
        .collect();

    // Group the positions by the type of char it is
    let antennas = antennas
        .iter()
        .fold(HashMap::new(), |mut acc_hashmap, tuple| {
            acc_hashmap
                .entry(tuple.1)
                .or_insert_with(Vec::new)
                .push(tuple.0);
            acc_hashmap
        });

    // Convert back to a Vec<Vec<Vector2>> as we don't care about the chars
    let antennas: Vec<_> = antennas.values().collect();

    let mut antinodes = HashSet::new();

    for group in antennas.iter() {
        for (&a, b) in group.iter().tuple_combinations() {
            let delta = b - a;
            antinodes.insert(a.clone());
            antinodes.insert(b.clone());
            for i in 1..row_len {
                antinodes.insert(a - (i * delta));
                antinodes.insert(b + (i * delta));
            }
        }
    }

    antinodes.retain(|node| node.x >= 0 && node.y >= 0 && node.x < col_len && node.y < row_len);

    println!("antinodes: {}", antinodes.len());
    // antinodes: 254
    // antinodes: 951
}
