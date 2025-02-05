use cgmath::Vector2;
use std::collections::HashMap;
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
                if self.get(Vec2::new(x, y)).is_some_and(|c0| c0 == c) {
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

    fn get(&self, v: Vec2) -> Option<char> {
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
                if self.get(Vec2::new(x, y)).is_some_and(|c0| c0 == c) {
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

fn solve(grid: &Grid) -> usize {
    let pos = grid.find('S').unwrap();
    let dir = Vec2::new(0, 1);

    // (position, direction) and the cost
    let mut visited: HashMap<(Vec2, Vec2), usize> = HashMap::new();
    // Current Nodes getting evaluated
    let mut open = vec![(pos, dir, 0)];

    while open.len() > 0 {
        let next = open.remove(0);
        let pos = next.0;
        let dir = next.1;
        let cost = next.2;

        // Check if we have already visisted this posistion
        let key = (pos, dir);
        if let Some(&prev_visit_cost) = visited.get(&key) {
            // Continue if the Cost is lower than the prevoius times that we have
            // visited the node
            if prev_visit_cost <= cost {
                continue;
            }
        }

        visited.insert(key, cost);

        let Some(c) = grid.get(pos) else {
            continue;
        };

        if c == 'E' {
            return cost;
        } else if c == '#' {
            // if its a wall continue
            continue;
        }

        // New nodes to look at
        // keep going in the same direction
        open.push((pos + dir, dir, cost + 1));
        // Can only turn 90 degrees each time
        let turn_dirs = turns(dir);
        open.push((pos + turn_dirs.0, turn_dirs.0, cost + 1001));
        open.push((pos + turn_dirs.1, turn_dirs.1, cost + 1001));
        /*
        println!("cost: {}", cost);
        let dir = turn(dir);
        open.push((pos + dir, dir, cost + 1001));
        println!("1cost: {}", cost);
        let dir = turn(dir);
        open.push((pos + dir, dir, cost + 1001));
        println!("2cost: {}", cost);
        let dir = turn(dir);
        open.push((pos + dir, dir, cost + 1001));
        println!("3cost: {}", cost);
        */
        // Sort by cost
        open.sort_by_key(|e| e.2);
    }
    panic!("solve failed");
}

fn turns(dir: Vec2) -> (Vec2, Vec2) {
    (Vec2::new(dir.y, -dir.x), Vec2::new(-dir.y, dir.x))
}

/*
// (0,1) x ==0 y == 1
// (1, 0) x ==1 y == 0
// (0, -1) x == 0 y== -1
// (-1, 0) x == -1 y ==0
// (0, 1)

fn turn(dir: Vec2) -> Vec2 {
    Vec2::new(dir.y, -dir.x)
}
*/

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("example2.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let data: Vec<_> = data.lines().collect();

    let grid = Grid::new(data.iter().map(|line| line.chars().collect()).collect());

    let score: usize = solve(&grid);
    println!("score: {}", score);
    // score: 75416
}
