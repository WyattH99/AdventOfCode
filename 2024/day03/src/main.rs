use std::fs;
use regex::Regex;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("data: {}", data);

    let dos: Vec<_> = data
        .match_indices("do()")
        .map(|(index, _)| index)
        .collect();

    let donts: Vec<_> = data
        .match_indices("don't()")
        .map(|(index, _)| index)
        .collect();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result: usize = re
        .captures_iter(&data)
        .filter(|capture| {
            let index = capture.get(0).unwrap().start();
            let last_do = dos.iter().filter(|&&x| x < index).max();
            let last_dont = donts.iter().filter(|&&x| x < index).max();
            if let (Some(last_do), Some(last_dont)) = (last_do, last_dont) {
                last_do > last_dont
            } else {
                last_dont.is_none()
            }
        })
        .map(|capture| 
            capture[1].parse::<usize>().unwrap() * 
            capture[2].parse::<usize>().unwrap())
        .sum();

    

    println!("result: {}", result);
    // result: 88802350
}
