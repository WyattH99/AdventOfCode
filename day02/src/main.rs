use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

/*
*
*/

fn safety(levels: Vec<usize>) -> bool {

    let increasing = levels.iter().is_sorted();
    let decreasing = levels.iter().is_sorted_by(|a, b| a >= b);

    let mut safe: bool = true;

    if increasing {
        println!("increasing");
        for (a, b) in levels.iter().tuple_windows() {
            let diff = b - a;
            if diff <= 0 || diff > 3 {
                println!("false: {} - {} = {}", a, b, diff);
                safe = false;
                break;
            }
        }
        if safe {
            println!("safe");
            return safe;
        }
            
    } else if decreasing {
        println!("decreasing");
        for (a, b) in levels.iter().tuple_windows() {
            let diff = a - b;
            if diff <= 0 || diff > 3 {
                println!("false: {} - {} = {}", a, b, diff);
                safe = false;
                break;
            }
        }
        if safe {
            println!("safe");
            return safe;
        }
    } else {
        println!("Mixed");
        safe = false;
    }
    safe
}


fn main() -> std::io::Result<()> {

    //let filename = "./example.txt";
    let filename = "./input.txt";
    let file = File::open(filename)?;
    let buf = BufReader::new(file);

    let mut result: i32 = 0;

    for line in buf.lines() {
        let levels: Vec<_> = line?.split_whitespace()
            .map(|s| s.parse::<usize>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        println!("{:?}", levels);

        if safety(levels.clone()) { 
            result += 1; 
        } else { // Problem Dampener
            for index in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(index);
                println!("new_levels: {:?}", new_levels);
                if safety(new_levels) {
                    result += 1;
                    break;
                }
            }
        }

        println!("Result: {}", result);
        println!("");

    }

    println!("Result: {}", result);
    // 672 is too high
    // Result: 349

    Ok(())
}
