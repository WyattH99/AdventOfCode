use std::cmp::max;
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

/*
* Secret number sequences:
* - Each evolves to the next
* - Calculate by:
*   - secret * 64 => mix => prune
*   - secret / 32 => floor => mix => prune
*   - secret * 2048 => mix => prune
*
* - mix: is bitwise xor
*  - 42 xor 15 => 37
* - prune: is modulo
*  - secret mod 16777216
*
*  The whole Calc:
*  sec = ((sec * 64) ^ sec) mod 16777216
*  sec = (((sec / 32) floor) ^ sec) mod 16777216
*  sec = ((sec * 2048) ^ sec) mod 16777216
*/

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("example2.txt").unwrap();
    //let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    /*
     *  The whole Calc:
     *  sec = ((sec * 64) ^ sec) mod 16777216
     *  sec = (((sec / 32) floor) ^ sec) mod 16777216
     *  sec = ((sec * 2048) ^ sec) mod 16777216
     */

    /*
    let mut sec: isize = 123;
    let mut sec_nums = vec![];
    let mut prices = vec![];
    let mut changes = vec![];
    sec_nums.push(sec);
    prices.push(sec % 10);
    for i in 0..10 {
        sec = ((sec * 64) ^ sec) % 16777216;
        sec = ((sec / 32) ^ sec) % 16777216;
        sec = ((sec * 2048) ^ sec) % 16777216;
        sec_nums.push(sec);
        prices.push(sec % 10);
        changes.push(prices[i + 1] - prices[i]);
    }
    for i in 0..sec_nums.len() {
        if i > 0 {
            println!("{} : {} : {}", sec_nums[i], prices[i], changes[i - 1]);
        } else {
            println!("{} : {}", sec_nums[i], prices[i]);
        }
    }

    let mut hash: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    prices.remove(0);
    let mut i = 3;
    for (a, b, c, d) in changes.iter().tuple_windows() {
        hash.entry((*a, *b, *c, *d))
            .and_modify(|v| *v += prices[i])
            .or_insert(prices[i]);
        i += 1;
        println!("{}, {}, {}, {}", a, b, c, d);
    }
    println!("hash: {:?}", hash);
    println!("highest: {}", hash.values().max().unwrap());
    */

    let buyers: Vec<isize> = data.lines().map(|x| x.parse().unwrap()).collect();
    println!("buyers: {:?}", buyers);
    let mut prices: Vec<Vec<isize>> = vec![Vec::new(); buyers.len()];
    let mut changes: Vec<Vec<isize>> = vec![Vec::new(); buyers.len()];
    let _temp: Vec<Vec<isize>> = buyers
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut sec: isize = *x;
            let mut sec_nums: Vec<isize> = vec![];
            for j in 0..2000 {
                sec = ((sec * 64) ^ sec) % 16777216;
                sec = ((sec / 32) ^ sec) % 16777216;
                sec = ((sec * 2048) ^ sec) % 16777216;
                sec_nums.push(sec);
                prices[i].push(sec % 10);
                if j > 0 {
                    changes[i].push(prices[i][j] - prices[i][j - 1]);
                }
            }
            sec_nums
        })
        .collect();

    // Hashmap key: last 4 change values, value: sum
    let mut hash_totals: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for buyer in 0..changes.len() {
        let mut j = 3;
        let mut seen: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
        for (a, b, c, d) in changes[buyer].iter().tuple_windows() {
            let key: (isize, isize, isize, isize) = (*a, *b, *c, *d);
            if key == (-2, 1, -1, 3) {
                println!("EX: Buyer: {}, Price: {}", buyers[buyer], prices[buyer][j]);
            }
            if seen.contains_key(&key) {
                continue;
            }
            seen.insert(key, prices[buyer][j + 1]);
            if !hash_totals.contains_key(&key) {
                hash_totals.insert(key, 0);
            }
            hash_totals.get_mut(&key).map(|val| {
                *val += prices[buyer][j + 1];
            });
            j += 1;
        }
        //println!("\nexample: {}", hash_totals.get(&(-2, 1, -1, 3)).unwrap());
        //println!("buyer highest: {}", hash_totals.values().max().unwrap());
    }
    println!("\nhighest total: {}", hash_totals.values().max().unwrap());
    // 1848 too high

    //println!("sum: {}", sec_nums.iter().sum::<isize>());
    // sum: 14869099597
}
