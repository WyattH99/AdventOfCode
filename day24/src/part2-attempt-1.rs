use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
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
    let x = x
        .iter()
        .fold(0, |acc, b| (acc << 1) | ((*b.1 == *"true") as u64));
    y.sort();
    y.reverse();
    let y = y
        .iter()
        .fold(0, |acc, b| (acc << 1) | ((*b.1 == *"true") as u64));
    let goal = x + y;
    println!("{:?} + {:?}", x, y);
    println!("Goal:\n{:?}", goal);
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

    // This doesn't work beacuse the random tuple combinations could
    // become infinite loops that have a side effect of stack overflow
    let formulas_copy = formulas.clone();
    let states_copy = states.clone();
    for (o1, o2, o3, o4) in formulas_copy.iter().tuple_combinations() {
        states = states_copy.clone();
        let sum = solve(&mut states, &formulas);
        //println!("\n{:?},{:?},{:?},{:?}", o1.0, o2.0, o3.0, o4.0);
        /*
                println!(
                    "{:?},{:?},{:?},{:?}",
                    formulas.get(o1.0).unwrap(),
                    formulas.get(o2.0).unwrap(),
                    formulas.get(o3.0).unwrap(),
                    formulas.get(o4.0).unwrap()
                );
        */
        //println!("{:?}", sum);
        if sum == goal {
            println!("{:?}", sum);
            println!("{:?},{:?},{:?},{:?}", o1.0, o2.0, o3.0, o4.0);
            break;
        }

        states = states_copy.clone();
        swap(o1, o2, o3, o4, &mut formulas);
        let sum = solve(&mut states, &formulas);
        //println!("{:?}", sum);
        if sum == goal {
            println!("{:?}", sum);
            println!("{:?},{:?},{:?},{:?}", o1.0, o2.0, o3.0, o4.0);
            break;
        }
        swap(o1, o2, o3, o4, &mut formulas);

        states = states_copy.clone();
        swap(o1, o3, o2, o4, &mut formulas);
        let sum = solve(&mut states, &formulas);
        //println!("{:?}", sum);
        if sum == goal {
            println!("{:?}", sum);
            println!("{:?},{:?},{:?},{:?}", o1.0, o2.0, o3.0, o4.0);
            break;
        }
        swap(o1, o3, o2, o4, &mut formulas);

        states = states_copy.clone();
        swap(o1, o4, o2, o3, &mut formulas);
        let sum = solve(&mut states, &formulas);
        //println!("{:?}", sum);
        if sum == goal {
            println!("{:?}", sum);
            println!("{:?},{:?},{:?},{:?}", o1.0, o2.0, o3.0, o4.0);
            break;
        }
        swap(o1, o4, o2, o3, &mut formulas);
    }
}

fn swap(
    o1: (&String, &(String, String, String)),
    o2: (&String, &(String, String, String)),
    o3: (&String, &(String, String, String)),
    o4: (&String, &(String, String, String)),
    formulas: &mut HashMap<String, (String, String, String)>,
) {
    let val1 = formulas.remove(o1.0).unwrap();
    let val2 = formulas.remove(o2.0).unwrap();
    formulas.insert(o1.0.to_string(), val2);
    formulas.insert(o2.0.to_string(), val1);
    let val1 = formulas.remove(o3.0).unwrap();
    let val2 = formulas.remove(o4.0).unwrap();
    formulas.insert(o3.0.to_string(), val2);
    formulas.insert(o4.0.to_string(), val1);
}

fn solve(
    states: &mut HashMap<String, String>,
    formulas: &HashMap<String, (String, String, String)>,
) -> u64 {
    let mut z: Vec<_> = Vec::new();
    let mut i = 0;
    loop {
        let key: String;
        if i < 10 {
            key = "z0".to_owned() + &i.to_string();
        } else {
            key = "z".to_owned() + &i.to_string();
        }
        if formulas.get(&key).is_none() {
            break;
        } else {
            let result = calc(key.to_string(), states, &formulas);
            z.push((key.clone(), result.clone()));
            i += 1;
        }
    }
    z.sort();
    z.reverse();
    let sum = z
        .iter()
        .fold(0, |acc, b| (acc << 1) | ((*b.1 == *"true") as u64));
    //println!("z: {:?}", z);
    sum
}

fn calc(
    wire: String,
    states: &mut HashMap<String, String>,
    formulas: &HashMap<String, (String, String, String)>,
) -> String {
    let result;
    let formula = formulas.get(&wire).unwrap();
    if states.get(&wire).is_some() {
        result = states.get(&wire).unwrap().to_string();
    } else {
        result = operators(states, formulas, formula);
        states.insert(wire.to_string(), result.clone());
    }
    //println!("{:?} = {:?} {:?}", formula, wire, result);
    result
}

fn operators(
    states: &mut HashMap<String, String>,
    formulas: &HashMap<String, (String, String, String)>,
    formula: &(String, String, String),
) -> String {
    let x;
    let op = formula.1.as_str();
    let y;
    if states.get(&formula.0).is_none() {
        x = calc(formula.0.clone(), states, formulas) == "true";
    } else {
        x = states.get(&formula.0).unwrap() == "true";
    }
    if states.get(&formula.2).is_none() {
        y = calc(formula.2.clone(), states, formulas) == "true";
    } else {
        y = states.get(&formula.2).unwrap() == "true";
    }
    let result: bool = match op {
        "AND" => x & y,
        "OR" => x | y,
        "XOR" => x ^ y,
        _ => panic!("invalid input: {:?}", formula),
    };
    //println!("{:?} {:?} {:?} {:?}", x, op, y, result);
    result.to_string()
}
