use std::fs;
use itertools::Itertools;

fn main() {

    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<_> = data.lines().collect();
    let split: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let rules: Vec<_> = split[0]
        .iter()
        .map(|line| {
            let mut num = line.split("|");
            (
                num.next().unwrap().parse::<usize>().unwrap(),
                num.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Vec<_>> = split[1]
        .iter()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let sum: usize = updates
        .iter()
        .filter(|update| 
            !update.iter()
                .combinations(2)
                .map(|v| (v[0], v[1]))
                .any(|(&x, &y)|
                    rules.iter()
                        .any(|r| r.1 == x && r.0 == y)
                )
        )
        .map(|update| update[update.len()/2])
        .sum();

    println!("sum: {:?}", sum);
    // sum: 5248
}
