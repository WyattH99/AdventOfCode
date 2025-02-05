use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() -> std::io::Result<()> {
    //let filename = "./example.txt";
    let filename = "./input.txt";
    let file = File::open(filename)?;
    let buf = BufReader::new(file);

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in buf.lines() {
        let line = line?; // Handle the Result from lines()
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap()
            .parse().unwrap());
        right.push(iter.next().unwrap()
            .parse().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right) 
        .map(|(l, r)| (l - r).abs())
        .sum();
    
    println!("{}", result); // 936063

    Ok(())
}
