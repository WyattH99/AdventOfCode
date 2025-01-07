use std::collections::HashMap;
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let stones: Vec<_> = data
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("stones: {:?}", stones);

    let blinks = 75;
    let mut mem = HashMap::new();
    let sum: usize = stones
        .iter()
        .map(|&number| get_stone_count(number, blinks, &mut mem))
        .sum();

    println!("blinks: {}    sum: {}", blinks, sum);
    // blinks: 25    sum: 220999
    // blinks: 75    sum: 261936432123724
}

// Recursive Function that takes the number and depth that is left and returns the number of stones
fn get_stone_count(number: usize, depth: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (number, depth);
    if mem.contains_key(&key) {
        *mem.get(&key).unwrap()
    } else {
        let result = if depth == 0 {
            return 1;
        } else {
            if number == 0 {
                return get_stone_count(1, depth - 1, mem);
            } else {
                // Check if it has even digits
                let s = number.to_string();
                if s.len() % 2 == 0 {
                    let l = s[..(s.len() / 2)].parse::<usize>().unwrap();
                    let r = s[(s.len() / 2)..].parse::<usize>().unwrap();
                    get_stone_count(l, depth - 1, mem) + get_stone_count(r, depth - 1, mem)
                } else {
                    get_stone_count(number * 2024, depth - 1, mem)
                }
            }
        };
        mem.insert(key, result);
        result
    }
}
