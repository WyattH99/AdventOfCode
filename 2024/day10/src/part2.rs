use std::collections::HashSet;
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}", data);

    // find all the trailheads and make a list of positions
    // the list of positions is the position of each 9 it can reach
    // recursive function to search all the trailheads and find all the 9s
    // could also do a BFS on this

    // make a 2D grid
    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();
    //println!("grid: {:?}", grid);

    // Find all the Trailheads in the grid
    let mut trailheads: Vec<_> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                trailheads.push((i as isize, j as isize));
            }
        }
    }

    let sum: usize = trailheads
        .iter()
        .map(|&p| {
            let mut nines = HashSet::new();
            let count = count_paths(p, &grid, 0, &mut HashSet::new(), &mut nines);
            //nines.len()
            count
        })
        .sum();

    println!("sum: {}", sum);
    // sum: 638
    // sum: 1289
}

fn count_paths(
    p: (isize, isize),
    grid: &[Vec<isize>],
    target: isize,
    visited: &mut HashSet<(isize, isize)>,
    nines: &mut HashSet<(isize, isize)>,
) -> usize {
    let n = grid.len();
    let m = grid[0].len();

    // Check to make sure we are still in the grid
    if p.0 < 0 || p.1 < 0 || p.0 >= n as isize || p.1 >= m as isize {
        return 0;
    }
    if visited.contains(&p) {
        return 0;
    }

    if grid[p.0 as usize][p.1 as usize] != target {
        return 0;
    }

    if target == 9 {
        nines.insert(p);
        return 1;
    }

    visited.insert(p);

    let mut count = 0;
    count += count_paths((p.0 + 1, p.1), grid, target + 1, visited, nines);
    count += count_paths((p.0 - 1, p.1), grid, target + 1, visited, nines);
    count += count_paths((p.0, p.1 + 1), grid, target + 1, visited, nines);
    count += count_paths((p.0, p.1 - 1), grid, target + 1, visited, nines);

    visited.remove(&p);

    count
}
