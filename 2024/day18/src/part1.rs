use cgmath::Vector2;
use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::fs;

type Vec2 = Vector2<isize>;

fn dirs() -> Vec<Vec2> {
    vec![
        Vec2::new(1, 0),
        Vec2::new(-1, 0),
        Vec2::new(0, 1),
        Vec2::new(0, -1),
    ]
}

//const GRID_LEN: isize = 6;
//const GRID_SIZE: isize = 7;
const GRID_LEN: isize = 70;
const GRID_SIZE: isize = 71;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let re = Regex::new(r"\d+").unwrap();
    let walls: Vec<isize> = re
        .captures_iter(data)
        .map(|cap| cap[0].parse().unwrap())
        .collect();

    let walls: Vec<_> = walls
        .chunks(2)
        .map(|chunk| Vec2::new(chunk[0], chunk[1]))
        .collect();
    //println!("walls: {:?}", walls);

    let start = Vec2::new(0, 0);
    let end = Vec2::new(GRID_LEN, GRID_LEN);

    let filter = &walls[..=1024];

    let result = dijkstra(
        &start,
        |&p| {
            let n: Vec<_> = dirs()
                .into_iter()
                .map(|dir| p + dir)
                .filter(|v| v.x >= 0 && v.y >= 0 && v.x < GRID_SIZE && v.y < GRID_SIZE)
                .filter(|v| !filter.contains(v))
                .map(|v| (v, 1))
                .collect();
            n
        },
        |&p| p == end,
    );

    println!("result: {}", result.expect("failed pathfinding").1);
    // result: 364
}
