use std::fs;
use regex::Regex;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("data: {}", data);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result: usize = re.captures_iter(&data)
                                .map(|capture| 
                                    capture[1].parse::<usize>().unwrap() * 
                                    capture[2].parse::<usize>().unwrap())
                                .sum();

    

    println!("result: {}", result);
    // result: 174336360
}
