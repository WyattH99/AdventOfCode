use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("{}", data);

    let data: Vec<_> = data.split("\n\n").collect();

    let mut locks: Vec<_> = Vec::new();
    let mut keys: Vec<_> = Vec::new();
    data.iter().for_each(|b| {
        if b.chars().next() == Some('#') {
            locks.push(b);
        } else {
            keys.push(b);
        }
    });

    let locks: Vec<Vec<Vec<_>>> = locks
        .iter()
        .map(|b| b.lines().map(|line| line.chars().collect()).collect())
        .collect();
    let keys: Vec<Vec<Vec<_>>> = keys
        .iter()
        .map(|b| b.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let height = locks[0].len() - 1;

    let locks: Vec<Vec<_>> = locks
        .iter()
        .map(|l| {
            let mut tmp: Vec<_> = vec![0; l[0].len()];
            for i in 1..l.len() {
                for j in 0..l[0].len() {
                    if l[i][j] == '#' {
                        tmp[j] += 1;
                    }
                }
            }
            tmp
        })
        .collect();

    let keys: Vec<Vec<_>> = keys
        .iter()
        .map(|k| {
            let mut tmp: Vec<_> = vec![0; k[0].len()];
            for i in 0..k.len() - 1 {
                for j in 0..k[0].len() {
                    if k[i][j] == '#' {
                        tmp[j] += 1;
                    }
                }
            }
            tmp
        })
        .collect();

    //println!("locks\n{:?}", locks);
    //println!("keys\n{:?}", keys);

    let sum = locks.iter().fold(0, |mut acc, lock| {
        keys.iter().for_each(|key| {
            let mut b: bool = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] >= height {
                    b = false;
                }
            }
            if b {
                acc += 1;
            }
        });
        acc
    });
    println!("{}", sum);
}
