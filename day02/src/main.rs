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
                    println!("false: {} - {} = {}", a, b, diff);
                    safe = false;
                    for index in 0..levels.len() {
                        let mut new_levels = levels.clone();
                        new_levels.remove(index);
                        println!("new_levels: {:?}", new_levels);
                        let mut new_safe: bool = true;
                        for (a, b) in new_levels.iter().tuple_windows() {
                            let new_diff = b - a;
                            if new_diff <= 0 || new_diff > 3 {
                                new_safe = false;
                                println!("new_false: {} - {} = {}", a, b, new_diff);
                            }
                        }
                        if new_safe {
                            result += 1;
                            println!("new_safe");
                            break;
                        }
                        
                    }
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
                    println!("false: {} - {} = {}", a, b, diff);
                    safe = false;
                    for index in 0..levels.len() {
                        let mut new_levels = levels.clone();
                        new_levels.remove(index);
                        println!("new_levels: {:?}", new_levels);
                        let mut new_safe: bool = true;
                        for (a, b) in new_levels.iter().tuple_windows() {
                            let new_diff = a - b;
                            if new_diff <= 0 || new_diff > 3 {
                                new_safe = false;
                                println!("new_false: {} - {} = {}", a, b, new_diff);
                            }
                        }
                        if new_safe {
                            result += 1;
                            println!("new_safe");
                            break;
                        }
                        
                    }
                }
            }
            if safe {
                result += 1;
                println!("safe");
            }
        } else {
            println!("Mixed");

            let mut safe_mix: bool = true;
            for index in 0..levels.len() {
                let mut new_sort = levels.clone();
                new_sort.remove(index);
                println!("new_sort: {:?}", new_sort);

                let increasing = new_sort.iter().is_sorted();
                let decreasing = new_sort.iter().is_sorted_by(|a, b| a >= b);

                if increasing {
                    println!("increasing");
                    let mut safe: bool = true;
                    for (a, b) in new_sort.iter().tuple_windows() {
                        let diff = b - a;
                        if diff <= 0 || diff > 3 {
                            println!("false: {} - {} = {}", a, b, diff);
                            safe = false;
                            for index in 0..new_sort.len() {
                                let mut new_levels = new_sort.clone();
                                new_levels.remove(index);
                                println!("new_levels: {:?}", new_levels);
                                let mut new_safe: bool = true;
                                for (a, b) in new_levels.iter().tuple_windows() {
                                    let new_diff = b - a;
                                    if new_diff <= 0 || new_diff > 3 {
                                        new_safe = false;
                                        println!("new_false: {} - {} = {}", a, b, new_diff);
                                    }
                                }
                                if new_safe {
                                    println!("new_safe");
                                    safe_mix = true;
                                    break;
                                }
                                
                            }
                        }
                    }
                    if safe {
                        safe_mix = true;
                        println!("safe");
                    }
                        
                } else if decreasing {
                    println!("decreasing");
                    let mut safe: bool = true;
                    for (a, b) in new_sort.iter().tuple_windows() {
                        let diff = a - b;
                        if diff <= 0 || diff > 3 {
                            println!("false: {} - {} = {}", a, b, diff);
                            safe = false;
                            for index in 0..new_sort.len() {
                                let mut new_levels = new_sort.clone();
                                new_levels.remove(index);
                                println!("new_levels: {:?}", new_levels);
                                let mut new_safe: bool = true;
                                for (a, b) in new_levels.iter().tuple_windows() {
                                    let new_diff = a - b;
                                    if new_diff <= 0 || new_diff > 3 {
                                        new_safe = false;
                                        println!("new_false: {} - {} = {}", a, b, new_diff);
                                    }
                                }
                                if new_safe {
                                    safe_mix = true;
                                    println!("new_safe");
                                    break;
                                }
                                
                            }
                        }
                    }
                    if safe {
                        safe_mix = true;
                        println!("safe");
                    }
                }
            }
            
            if safe_mix {
                result += 1;
            }




        }
        println!("Result: {}", result);
        println!("");


    }

    println!("Result: {}", result);
    // 672 is too high
    // Result: 

    Ok(())
}
