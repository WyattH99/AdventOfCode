use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;


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

        let increasing = levels.iter().is_sorted();
        let decreasing = levels.iter().is_sorted_by(|a, b| a >= b);

        if increasing {
            println!("increasing");
            let mut safe: bool = true;
            for (a, b) in levels.iter().tuple_windows() {
                let diff = b - a;
                if diff <= 0 || diff > 3 {
                    safe = false;
                    println!("false: {} - {} = {}", a, b, diff);
                }
            }
            if safe {
                result += 1;
                println!("safe");
            }
                
        } else if decreasing {
            println!("decreasing");
            let mut safe: bool = true;
            for (a, b) in levels.iter().tuple_windows() {
                let diff = a - b;
                if diff <= 0 || diff > 3 {
                    safe = false;
                    println!("false: {} - {} = {}", a, b, diff);
                }
            }
            if safe {
                result += 1;
                println!("safe");
            }
        } else {
            println!("Mixed");
        }
        println!("Result: {}", result);
        println!("");


    }

    println!("Result: {}", result);
    // 153 is too low
    // Result: 282

    Ok(())
}
