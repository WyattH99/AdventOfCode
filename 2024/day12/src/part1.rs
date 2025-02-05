use cgmath::Vector2;
use std::collections::HashMap;
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

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("example2.txt").unwrap();
    //let data = fs::read_to_string("example3.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: \n{}", data);

    let grid = data
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, letter)| (Vec2::new(i as isize, j as isize), letter))
        })
        .collect();

    let regions = split_into_regions(grid);
    //println!("\nregions: {:?}", regions);

    let sum: usize = regions
        .iter()
        .map(|region| {
            let area = region.len();
            let perimeter = get_perimeter(region.to_vec());
            area * perimeter
        })
        .sum();
    println!("\nsum: {}", sum);
    //sum: 1424472
}

fn split_into_regions(mut grid: HashMap<Vec2, char>) -> Vec<Vec<Vec2>> {
    let mut regions = vec![];
    while let Some((&pos, &c)) = grid.iter().next() {
        let mut region = vec![];
        collect_region(pos, c, &mut grid, &mut region);
        regions.push(region);
    }
    regions
}

fn collect_region(pos: Vec2, c: char, grid: &mut HashMap<Vec2, char>, region: &mut Vec<Vec2>) {
    // Check to see if pos and c are in the map
    if let Some(&c1) = grid.get(&pos) {
        if c1 == c {
            grid.remove(&pos);
            region.push(pos);
            for dir in dirs().into_iter() {
                collect_region(pos + dir, c, grid, region);
            }
        }
    }
}

fn get_perimeter(region: Vec<Vec2>) -> usize {
    let perimeter = region
        .iter()
        .map(|pos| {
            let mut perimeter = 0;
            for dir in dirs().into_iter() {
                if !region.contains(&(pos + dir)) {
                    perimeter += 1;
                }
            }
            perimeter
        })
        .sum();
    perimeter
}
//
// First Attempt and Thoughts
//
// Create a 2D Grid
// HashMap of each region letter as the key
//  Value is a tuple (parimeter, area)
// Go through each plant checking the u, d, l, r
// incrementing the area for each
// and incrementing the parimeter for each if the u, d, l, r is not the same plant
//
// This doesn't work as there is more than one region for each of the plants
// I have to treat each region as a graph
// maybe do a flood fill for each region
// while keeping track of each letter I have seen
// that way we can identify when I am encountering a new region
/*
    // 2D grid
    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let row_len = grid.len();
    let col_len = grid[0].len();

    // 2D hashmap (letter, (parimeter, area))
    let mut regions: HashMap<char, (isize, isize)> = HashMap::new();
    for r in 0..row_len {
        for c in 0..col_len {
            let letter = grid[r][c];
            //regions.insert(grid[r][c], (0, 0));
            // iterate the area
            let tup = regions.entry(letter).or_insert((0, 0));
            *tup = (tup.0 + 4, tup.1 + 1);
            // Check the 4 sides and decrement the perimeter for each that is different
            let mut decrement = 0;
            if r + 1 < row_len && grid[r + 1][c] == letter {
                decrement += 1;
            }
            if r as isize - 1 >= 0 && grid[r - 1][c] == letter {
                decrement += 1;
            }
            if c + 1 < col_len && grid[r][c + 1] == letter {
                decrement += 1;
            }
            if c as isize - 1 >= 0 && grid[r][c - 1] == letter {
                decrement += 1;
            }
            /*
                        println!(
                            "letter: {}, tup.0: {}, decrement: {}",
                            letter, tup.0, decrement
                        );
            */
            *tup = (tup.0 - decrement, tup.1);
        }
    }

    println!("regions: {:?}", regions);

    let sum: isize = regions.iter().map(|(_k, (p, a))| p * a).sum();
    println!("\nsum: {}", sum);
*/
