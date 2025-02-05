use cgmath::Vector2;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Vec2 = Vector2<isize>;

struct Grid {
    n: isize,
    m: isize,
    data: Vec<Vec<char>>,
}

#[allow(dead_code)]
impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        let n = data.len() as isize;
        let m = data[0].len() as isize;
        Self { n, m, data }
    }

    fn find(&self, c: char) -> Option<Vec2> {
        for x in 0..self.n {
            for y in 0..self.m {
                if self.get(&Vec2::new(x, y)).is_some_and(|c0| c0 == c) {
                    return Some(Vec2::new(x, y));
                }
            }
        }
        None
    }

    fn set(&mut self, v: Vec2, c: char) {
        let x = v.x as usize;
        let y = v.y as usize;
        self.data[x][y] = c;
    }

    fn get(&self, v: &Vec2) -> Option<char> {
        if v.x >= 0 && v.y >= 0 {
            let x = v.x as usize;
            let y = v.y as usize;
            if let Some(&c) = self.data.get(x).and_then(|r| r.get(y)) {
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn find_all(&self, c: char) -> Vec<Vec2> {
        let mut res = vec![];

        for x in 0..self.n {
            for y in 0..self.m {
                if self.get(&Vec2::new(x, y)).is_some_and(|c0| c0 == c) {
                    res.push(Vec2::new(x, y));
                }
            }
        }
        res
    }

    fn _debug_print(&self) {
        for r in 0..self.n {
            let str: String = (0..self.m)
                .map(|i| self.data[r as usize][i as usize])
                .collect();
            println!("{}", str);
        }
        println!();
    }
}

fn dirs() -> Vec<Vec2> {
    vec![
        Vec2::new(1, 0),
        Vec2::new(-1, 0),
        Vec2::new(0, 1),
        Vec2::new(0, -1),
    ]
}

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let data: Vec<_> = data.lines().collect();
    let grid = Grid::new(data.iter().map(|line| line.chars().collect()).collect());

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    let mut pos = start;

    let mut cost = 0;
    let mut costs = HashMap::new();

    costs.insert(start, cost);
    cost += 1;

    'outer: while pos != end {
        for new_pos in dirs()
            .into_iter()
            .map(|dir| dir + pos)
            .filter(|new_pos| grid.get(new_pos).is_some_and(|c| c != '#'))
        {
            if !costs.contains_key(&new_pos) {
                costs.insert(new_pos, cost);
                cost += 1;
                pos = new_pos;
                continue 'outer;
            }
        }
    }

    let jumps = {
        let mut v = vec![];
        for x in -2..=2 as isize {
            for y in -2..=2 as isize {
                let l = x.abs() + y.abs();
                if l > 1 && l <= 2 {
                    v.push(Vec2::new(x, y));
                }
            }
        }
        v
    };

    let mut set = HashSet::new();
    for (&cheat_start, _) in costs.iter() {
        for cheat_end in jumps.iter().map(|jump| jump + cheat_start) {
            if let Some(saved) = get_cheat_score(cheat_start, cheat_end, &costs) {
                let key = (cheat_start, cheat_end);
                if saved >= 100 {
                    set.insert(key);
                }
            }
        }
    }

    println!("set: {}", set.len());
    // set: 1441
}

fn get_cheat_score(
    cheat_start: Vec2,
    cheat_end: Vec2,
    costs: &HashMap<Vec2, isize>,
) -> Option<isize> {
    if !costs.contains_key(&cheat_end) {
        return None;
    }

    let delta = cheat_end - cheat_start;
    let shortcut_length = delta.x.abs() + delta.y.abs();
    let saved = costs[&cheat_end] - costs[&cheat_start] - shortcut_length;
    if saved > 0 {
        return Some(saved);
    } else {
        return None;
    }
    None
}
