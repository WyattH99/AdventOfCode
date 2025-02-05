use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y}
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn right(&self) -> Self {
        Position::new(self.y, -self.x)
    }
}

struct Grid {
    n: isize, // x size of data
    m: isize, // y size of data
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        let n = data.len() as isize;
        let m = data[0].len() as isize;
        Self { n, m, data }
    }

    fn get(&self, pos: &Position) -> Option<char> {
        if pos.x < 0 || pos.y < 0 || pos.x > self.n - 1 || pos.y > self.m - 1 {
            None  // out of bounds
        } else {
            Some(self.data[pos.x as usize][pos.y as usize])
        }
    }

    fn find(&self, c: char) -> Position {
        for i in 0..self.n - 1 {
            for j in 0..self.m - 1 {
                let pos = Position::new( i as isize, j as isize);
                if self.get(&pos) == Some(c) {
                    return pos;
                }
            }
        }
        panic!();
    }
}


fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("data: {:?}", data);

    let grid = Grid::new(data
        .lines()
        .map(|line| line.chars().collect())
        .collect());

    //println!("{:?}", grid.data);

    let mut visited = HashSet::new();

    let mut pos = grid.find('^');
    let mut dir = Position::new(-1,0);

    loop {
        visited.insert(pos);
        let next = pos.add(&dir);
        if let Some(c) = grid.get(&next) {
            if c == '#' { // hit a wall
                dir = dir.right();
            } else {
                pos = next; // continue in the direction
            }
        } else {
            break; // out of bounds
        }

    }


    println!("distinct positions: {}", visited.len());
    // 5162
}
