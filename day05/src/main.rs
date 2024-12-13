use std::fs;
use itertools::Itertools;

fn main() {

    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<_> = data.lines().collect();
    let split: Vec<_> = lines.split(|line| line.is_empty()).collect();
    println!("rules: {:?}\n", split);

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
    println!("rules: {:?}\n", rules);

    let updates: Vec<Vec<_>> = split[1]
        .iter()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    println!("updates: {:?}\n", updates);

    let incorrect_updates: Vec<Vec<_>> = updates
        .into_iter()
        .filter(|update| 
            update.iter()
                .combinations(2)
                .map(|v| (v[0], v[1]))
                .any(|(&x, &y)|
                    rules.iter()
                        .any(|r| r.1 == x && r.0 == y)
                )
        ).collect();
    println!("incorrect_updates: {:?}\n", incorrect_updates);

    let fixed_updates: Vec<Vec<_>> = incorrect_updates
        .into_iter()
        .map(|update| {
            let mut update = update.clone();
            while let Some((i0, i1)) = update
                .iter()
                .combinations(2)
                .map(|v| (v[0], v[1]))
                .find_map(|(&x, &y)| { rules
                    .iter()
                    .find(|r| r.1 == x && r.0 == y).map(|r| {
                        (
                            update.iter().position(|&e| e == r.1).unwrap(),
                            update.iter().position(|&e| e == r.0).unwrap(),
                        )
                    }) 
                })
            {
                update.swap(i0, i1);
            }
            update
        }
        )
        .collect();
    println!("fixed_updates: {:?}\n", fixed_updates);

    let sum: usize = fixed_updates
        .into_iter()
        .map(|update| update[update.len()/2])
        .sum();

    println!("sum: {:?}\n", sum);
    // sum: 
}
