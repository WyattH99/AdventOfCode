use cgmath::Vector2;
use std::fs;

type Vec2 = Vector2<isize>;

struct Grid {
    n: isize,
    m: isize,
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        let n = data.len() as isize;
        let m = data[0].len() as isize;
        Self { n, m, data }
    }

    fn find(&mut self, c: char) -> Option<Vec2> {
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

fn move_to_dir(c: char) -> Vec2 {
    match c {
        '<' => Vec2::new(0, -1),
        '>' => Vec2::new(0, 1),
        '^' => Vec2::new(-1, 0),
        'v' => Vec2::new(1, 0),
        _ => panic!("move_to_dir"),
    }
}

impl Grid {
    fn try_move_robot(&mut self, c: char) {
        let dir = move_to_dir(c);
        let pos = self.find('@').unwrap();

        if self.can_push(pos, dir) {
            self.push(pos, dir);
        }
    }

    fn can_push(&self, pos: Vec2, dir: Vec2) -> bool {
        let next = pos + dir;
        let next_c = self.get(next).unwrap();
        match next_c {
            '.' => true,
            '#' => false,
            _ => self.can_push(next, dir),
        }
    }

    fn push(&mut self, pos: Vec2, dir: Vec2) {
        let next = pos + dir;
        let next_c = self.get(next).unwrap();
        let current_c = self.get(pos).unwrap();
        if next_c != '.' {
            self.push(next, dir);
        }
        self.set(next, current_c);
        self.set(pos, '.');
    }
}

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("example2.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}\n", data);

    let data: Vec<_> = data.lines().collect();
    let data: Vec<_> = data.split(|line| line.is_empty()).collect();

    let mut grid = Grid::new(data[0].iter().map(|line| line.chars().collect()).collect());
    let moves: Vec<_> = data[1]
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for &c in moves.iter() {
        grid.try_move_robot(c);
    }

    let all_boxes = grid.find_all('O');

    let sum: isize = all_boxes.iter().map(|b| 100 * b.x + b.y).sum();
    println!("sum: {}", sum);
    // sum: 1568399
}
