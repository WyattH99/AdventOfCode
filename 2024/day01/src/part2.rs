use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() -> std::io::Result<()> {
    //let filename = "./example.txt";
    let filename = "./input.txt";
    let file = File::open(filename)?;
    let buf = BufReader::new(file);

    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for line in buf.lines() {
        let line = line?; // Handle the Result from lines()
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap()
            .parse().unwrap());
        right.push(iter.next().unwrap()
            .parse().unwrap());
    }

    let result: usize = left.iter()
        .map(|x| x * right.iter()
            .filter(|y| &x == y).count())
        .sum();

    println!("{}", result);
    // 23150395    

    Ok(())
}
