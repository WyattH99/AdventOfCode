use std::collections::HashMap;
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let data: Vec<_> = data.lines().collect();
    let data: Vec<_> = data.split(|line| line.is_empty()).collect();
    println!("data: {:?}", data);

    let towels: Vec<_> = data[0][0].split(",").map(|s| s.trim()).collect();

    let patterns = data[1];

    let possible_designs: usize = patterns
        .iter()
        .map(|pattern| count_make_pattern(pattern, &towels, &mut HashMap::new()))
        .sum();

    println!("\npossible_designs: {}", possible_designs);
    // possible_designs: 285
    // possible_designs: 636483903099279
}

fn count_make_pattern(pattern: &str, towels: &[&str], mem: &mut HashMap<String, usize>) -> usize {
    if mem.contains_key(pattern) {
        return mem[pattern];
    }
    let result = towels
        .iter()
        .map(|&towel| {
            if pattern == towel {
                1
            } else if pattern.starts_with(towel) {
                let subpattern = &pattern[towel.len()..];
                count_make_pattern(subpattern, towels, mem)
            } else {
                0
            }
        })
        .sum();

    mem.insert(pattern.into(), result);
    println!("mem: {:?}\n", mem);
    result
}
