use cgmath::Vector2;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::iter::repeat;

type Vec2 = Vector2<isize>;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let codes: Vec<_> = data.lines().collect();
    /*
        let code = "029A";
        let len = get_numpad_code_length(code);
        println!("code len: {}", len);
    */

    let result: usize = codes
        .iter()
        .map(|code| {
            let len = get_numpad_code_length(code, 25, &mut HashMap::new());
            let numeric: usize = code[..(code.len() - 1)].parse().unwrap();
            len * numeric
        })
        .sum();
    println!("result: {}", result);
    //result: 223838 too high
    //result: 219254
    //result: 264518225304496
}

fn get_numpad_code_length(
    code: &str,
    depth: usize,
    mem: &mut HashMap<(String, usize), usize>,
) -> usize {
    let mut chars: Vec<char> = code.chars().collect();
    chars.insert(0, 'A');

    chars
        .iter()
        .tuple_windows()
        .map(|(&from, &to)| {
            let paths = &NUMERIC_SHORTEST_PATH[&(from, to)];
            paths
                .iter()
                .map(|path| get_keypad_code_length(path, depth, mem))
                .min()
                .unwrap()
        })
        .sum()
}

fn get_keypad_code_length(
    path: &str,
    depth: usize,
    mem: &mut HashMap<(String, usize), usize>,
) -> usize {
    let mem_key = (path.to_string(), depth);
    if mem.contains_key(&mem_key) {
        return mem[&mem_key];
    }

    let mut chars: Vec<char> = path.chars().collect();

    chars.insert(0, 'A');
    chars.push('A');

    let result = if depth == 1 {
        chars
            .iter()
            .tuple_windows()
            .map(|(&from, &to)| {
                if from != to {
                    let paths = &KEYPAD_SHORTEST_PATH[&(from, to)];
                    paths[0].len() + 1
                } else {
                    1
                }
            })
            .sum()
    } else {
        chars
            .iter()
            .tuple_windows()
            .map(|(&from, &to)| {
                if from != to {
                    let paths = &KEYPAD_SHORTEST_PATH[&(from, to)];
                    paths
                        .iter()
                        .map(|path| get_keypad_code_length(path, depth - 1, mem))
                        .min()
                        .unwrap()
                } else {
                    1
                }
            })
            .sum()
    };
    mem.insert(mem_key, result);
    result
}

static NUMERIC_SHORTEST_PATH: Lazy<HashMap<(char, char), Vec<String>>> = Lazy::new(|| {
    let keys: Vec<char> = NUMPAD.keys().map(|c| *c).collect();

    let mut paths = HashMap::new();

    keys.iter().tuple_combinations().for_each(|(&from, &to)| {
        paths.insert((from, to), get_numeric_shortest_path(from, to));
        paths.insert((to, from), get_numeric_shortest_path(to, from));
    });

    paths
});

static KEYPAD_SHORTEST_PATH: Lazy<HashMap<(char, char), Vec<String>>> = Lazy::new(|| {
    let keys: Vec<char> = KEYPAD.keys().map(|c| *c).collect();

    let mut paths = HashMap::new();

    keys.iter().tuple_combinations().for_each(|(&from, &to)| {
        paths.insert((from, to), get_keypad_shortest_path(from, to));
        paths.insert((to, from), get_keypad_shortest_path(to, from));
    });

    paths
});

static NUMPAD: Lazy<HashMap<char, Vec2>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('7', Vec2::new(0, 0));
    map.insert('8', Vec2::new(1, 0));
    map.insert('9', Vec2::new(2, 0));

    map.insert('4', Vec2::new(0, 1));
    map.insert('5', Vec2::new(1, 1));
    map.insert('6', Vec2::new(2, 1));

    map.insert('1', Vec2::new(0, 2));
    map.insert('2', Vec2::new(1, 2));
    map.insert('3', Vec2::new(2, 2));

    map.insert('0', Vec2::new(1, 3));
    map.insert('A', Vec2::new(2, 3));

    map
});

static KEYPAD: Lazy<HashMap<char, Vec2>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('^', Vec2::new(1, 0));
    map.insert('A', Vec2::new(2, 0));

    map.insert('<', Vec2::new(0, 1));
    map.insert('v', Vec2::new(1, 1));
    map.insert('>', Vec2::new(2, 1));

    map
});

fn get_numeric_shortest_path(from: char, to: char) -> Vec<String> {
    let start = &NUMPAD[&from];
    let end = &NUMPAD[&to];

    let delta = end - start;

    let horizontal = if delta.x > 0 { '>' } else { '<' };
    let vertical = if delta.y > 0 { 'v' } else { '^' };

    let x = delta.x.abs() as usize;
    let y = delta.y.abs() as usize;

    if (from == 'A' || from == '0') && (to == '7' || to == '4' || to == '1') {
        vec![repeat(vertical)
            .take(y)
            .chain(repeat(horizontal).take(x))
            .collect()]
    } else if (to == 'A' || to == '0') && (from == '7' || from == '4' || from == '1') {
        vec![repeat(horizontal)
            .take(x)
            .chain(repeat(vertical).take(y))
            .collect()]
    } else if x > 0 && y > 0 {
        vec![
            repeat(horizontal)
                .take(x)
                .chain(repeat(vertical).take(y))
                .collect(),
            repeat(vertical)
                .take(y)
                .chain(repeat(horizontal).take(x))
                .collect(),
        ]
    } else {
        vec![repeat(horizontal)
            .take(x)
            .chain(repeat(vertical).take(y))
            .collect()]
    }
}

fn get_keypad_shortest_path(from: char, to: char) -> Vec<String> {
    let start = &KEYPAD[&from];
    let end = &KEYPAD[&to];

    let delta = end - start;

    let horizontal = if delta.x > 0 { '>' } else { '<' };
    let vertical = if delta.y > 0 { 'v' } else { '^' };

    let x = delta.x.abs() as usize;
    let y = delta.y.abs() as usize;

    if (from == 'A' || from == '^') && (to == '<') {
        vec![repeat(vertical)
            .take(y)
            .chain(repeat(horizontal).take(x))
            .collect()]
    } else if (to == 'A' || to == '^') && (from == '<') {
        vec![repeat(horizontal)
            .take(x)
            .chain(repeat(vertical).take(y))
            .collect()]
    } else if x > 0 && y > 0 {
        vec![
            repeat(horizontal)
                .take(x)
                .chain(repeat(vertical).take(y))
                .collect(),
            repeat(vertical)
                .take(y)
                .chain(repeat(horizontal).take(x))
                .collect(),
        ]
    } else {
        vec![repeat(horizontal)
            .take(x)
            .chain(repeat(vertical).take(y))
            .collect()]
    }
}
