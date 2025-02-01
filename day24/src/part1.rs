use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("example2.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}", data);

    let data: Vec<_> = data.split("\n\n").collect();

    let mut states: HashMap<String, String> = HashMap::new();
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
            states.insert(wire, state);
        });
    });
    println!("states: {:?}\n", states);

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
            let result = calc(key.to_string(), &mut states, &formulas);
            z.push((key.clone(), result.clone()));
            println!(
                "key: {}, formula: {:?}, result: {}\n",
                key,
                formulas.get(&key),
                result
            );
            i += 1;
        }
    }

    z.sort();
    z.reverse();
    let z: Vec<_> = z.iter().filter(|res| res.0.contains("z")).collect();
    println!("");
    println!("{:?}", z);
    let result = z
        .iter()
        .fold(0, |acc, b| (acc << 1) | ((*b.1 == *"true") as u64));
    println!("");
    println!("{:?}", result);
    // 50411513338638
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
    println!("{:?} = {:?} {:?}", formula, wire, result);
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
