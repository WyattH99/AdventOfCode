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

    let blinks = 25;
    let sum: usize = stones
        .iter()
        .map(|&number| get_stone_count(number, blinks))
        .sum();

    println!("blinks: {}    sum: {}", blinks, sum);
    // blinks: 25    sum: 220999
}

// Recursive Function that takes the number and depth that is left and returns the number of stones
fn get_stone_count(number: usize, depth: i32) -> usize {
    if depth == 0 {
        return 1;
    } else {
        if number == 0 {
            return get_stone_count(1, depth - 1);
        } else {
            // Check if it has even digits
            let s = number.to_string();
            if s.len() % 2 == 0 {
                let l = s[..(s.len() / 2)].parse::<usize>().unwrap();
                let r = s[(s.len() / 2)..].parse::<usize>().unwrap();
                get_stone_count(l, depth - 1) + get_stone_count(r, depth - 1)
            } else {
                get_stone_count(number * 2024, depth - 1)
            }
        }
    }
}
