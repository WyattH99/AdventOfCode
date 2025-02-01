//use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("example2.txt").unwrap();
    //let data = fs::read_to_string("example3.txt").unwrap();
    //let data = fs::read_to_string("example4.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}", data);

    let data: Vec<_> = data.split("\n\n").collect();

    let mut states: HashMap<String, String> = HashMap::new();
    let mut x: Vec<_> = Vec::new();
    let mut y: Vec<_> = Vec::new();
    let re = Regex::new(r"([a-z]+\d+): (\d+)").unwrap();
    data[0].lines().for_each(|line| {
        re.captures_iter(line).for_each(|cap| {
            let wire = cap[1].to_string();
            let state;
            if cap[2].to_string() == "1" {
                state = true.to_string();
            } else {
                state = false.to_string();
            }
            if wire.contains("x") {
                x.push((wire.clone(), state.clone()));
            }
            if wire.contains("y") {
                y.push((wire.clone(), state.clone()));
            }
            states.insert(wire, state);
        });
    });
    x.sort();
    x.reverse();
    let _x = x
        .iter()
        .fold(0, |acc, b| (acc << 1) | ((*b.1 == *"true") as u64));
    y.sort();
    y.reverse();
    let _y = y
        .iter()
        .fold(0, |acc, b| (acc << 1) | ((*b.1 == *"true") as u64));
    //let goal = x + y;
    //println!("{:?} + {:?}", x, y);
    //println!("Goal:\n{:?}", goal);
    // 86: 1010110
    // 40: 101000

    let re = Regex::new(r"([a-z]*[0-9]*) (AND|OR|XOR) ([a-z]*[0-9]*) -> ([a-z]*[0-9]*)").unwrap();
    let mut formulas: HashMap<String, (String, String, String)> = HashMap::new();
    data[1].lines().for_each(|line| {
        re.captures_iter(line).for_each(|cap| {
            formulas.insert(
                cap[4].to_string(),
                (cap[1].to_string(), cap[2].to_string(), cap[3].to_string()),
            );
        });
    });

    // iterate through the formulas and see if i can find gates that don't follow the similar
    // structure as a full-adder
    let mut ans: HashSet<_> = HashSet::new();

    formulas.iter().for_each(|(out, (in1, op, in2))| {
        // Check each Z comes from an XOR
        if out.starts_with("z") && *op != "XOR".to_string() {
            if out != "z45" {
                ans.insert(out.to_string());
            }
        }

        // XOR that doesn't have a Z out should have a X and Y input
        if *op == "XOR".to_string()
            && !out.starts_with("z")
            && !(in1.starts_with("x") || in1.starts_with("y"))
            && !(in2.starts_with("x") || in2.starts_with("y"))
        {
            ans.insert(out.to_string());
            //println!("{} = {} {} {}", out, in1, op, in2);
        }

        // All OR ins should be outputs of ANDs
        if *op == "OR".to_string() {
            if formulas.get(in1).unwrap().1 != "AND".to_string() {
                ans.insert(in1.to_string());
                //println!("{} = {} {} {}", out, in1, op, in2);
            }
            if formulas.get(in2).unwrap().1 != "AND".to_string() {
                ans.insert(in2.to_string());
                //println!("{} = {} {} {}", out, in1, op, in2);
            }
        }
        // All AND ins that is not X and Y should be outputs of XOR or OR
        if *op == "AND".to_string()
            && !(in1.starts_with("x") || in1.starts_with("y"))
            && !(in2.starts_with("x") || in2.starts_with("y"))
        {
            if formulas.get(in1).unwrap().1 == "AND".to_string() {
                ans.insert(in1.to_string());
                println!("{} = {} {} {}", out, in1, op, in2);
            }
            if formulas.get(in2).unwrap().1 == "AND".to_string() {
                ans.insert(in2.to_string());
                println!("{} = {} {} {}", out, in1, op, in2);
            }
        }

        // If Z XOR then one of the ins should be an output of another XOR
        /*
                if out.starts_with("z")
                    && *op == "XOR".to_string()
                    && !(in1.starts_with("x") || in1.starts_with("y"))
                    && !(in2.starts_with("x") || in2.starts_with("y"))
                {
                    if formulas.get(in1).unwrap().1 != "XOR".to_string() && {
                        ans.push(in1.to_string());
                        println!("{} = {} {} {}", out, in1, op, in2);
                    }
                    if formulas.get(in2).unwrap().1 == "AND".to_string() {
                        ans.push(in2.to_string());
                        println!("{} = {} {} {}", out, in1, op, in2);
                    }
                }


        */
    });

    let mut ans: Vec<_> = ans.into_iter().collect();
    ans.sort();
    println!("ans: {:?}", ans.join(","));
    // kfs,kqn,tqm,vwr,z06,z11,z16,z45 WRONG
    // hcn,kfs,tqm,vwr,z06,z11,z16,z45 WRONG
    // hcm,kfs,kqn,tqm,vwr,z06,z11,z16 WRONG
    // gfv,hcm,kfs,tqm,vwr,z06,z11,z16
}
