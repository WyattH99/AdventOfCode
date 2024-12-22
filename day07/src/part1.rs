use std::fs;


fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("data: {:?}", data);

    let sum: usize = data
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let target = split.next().unwrap().parse::<usize>().unwrap();
            let numbers: Vec<_> = split.next().unwrap()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (target, numbers)
        }).filter(|e| can_solve(e.0, &e.1))
        .map(|e| e.0)
        .sum();

    println!("sum: {}", sum);
    // 5512534574980
}

fn can_solve(target: usize, numbers: &[usize]) -> bool{
    let mut results = vec![numbers[0]];
    for number in &numbers[1..] {
        results = results.iter()
            .flat_map(|result| vec![result*number, result+number]).collect();
    }
    results.iter().any(|&result| result == target)
}
