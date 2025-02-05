use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn step(mut sec: isize) -> isize {
    sec = (sec ^ (sec * 64)) % 16777216;
    sec = (sec ^ (sec / 32)) % 16777216;
    sec = (sec ^ (sec * 2048)) % 16777216;
    sec
}

fn price(sec: isize) -> isize {
    sec % 10
}

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("example2.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let mut totals: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();

    data.lines()
        .map(|line| line.parse::<isize>().unwrap())
        .for_each(|mut num| {
            let mut prices: Vec<isize> = vec![];
            prices.push(price(num));
            for _ in 0..2000 {
                num = step(num);
                prices.push(price(num));
            }
            let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
            for (a, b, c, d, e) in prices.iter().tuple_windows() {
                let changes: (isize, isize, isize, isize) = (b - a, c - b, d - c, e - d);
                if seen.contains(&changes) {
                    continue;
                }
                seen.insert(changes);
                if !totals.contains_key(&changes) {
                    totals.insert(changes, 0);
                }
                totals.get_mut(&changes).map(|val| {
                    *val += e;
                });
            }
        });

    println!("max: {}", totals.values().max().unwrap());
    // max: 1717
}
