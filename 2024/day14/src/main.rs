use cgmath::{InnerSpace, Vector2};
use itertools::Itertools;
use regex::Regex;
use std::{fs, thread, time::Duration};

type Vec2 = Vector2<isize>;

//const WIDTH: isize = 11;
//const HEIGHT: isize = 7;
const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let r = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let robots: Vec<_> = data
        .lines()
        .map(|line| {
            let capture = r.captures_iter(line).next().unwrap();
            (
                Vec2::new(capture[1].parse().unwrap(), capture[2].parse().unwrap()),
                Vec2::new(capture[3].parse().unwrap(), capture[4].parse().unwrap()),
            )
        })
        .collect();

    let mut i = 0;

    loop {
        // tick the robots
        let robots: Vec<_> = robots
            .iter()
            .map(|robot| {
                // robot.0 is position
                // robot.1 is velocity
                let pos = robot.0 + robot.1 * i;
                Vec2::new(pos.x.rem_euclid(WIDTH), pos.y.rem_euclid(HEIGHT))
            })
            .collect();

        if robots
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                let m = (**a - *b).magnitude2();
                m == 1 || m == 2
            })
            .count()
            > 750
        {
            print_grid(&robots);
            println!("tick: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        i += 1;
    }
    //tick: 7569
}

fn print_grid(robots: &Vec<Vec2>) {
    let mut grid = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = 'R';
        //grid[robot.0.y as usize][robot.0.x as usize] = 'R';
        // Doesn't matter to keep track of the overlapping
        /*
        let c = grid[robot.0.y as usize][robot.0.x as usize];
                if c == '.' {
                    grid[robot.0.y as usize][robot.0.x as usize] = '1';
                } else {
                    let mut num = c.to_digit(10).unwrap() as usize;
                    num += 1;
                    grid[robot.0.y as usize][robot.0.x as usize] =
                        char::from_digit(num as u32, 10).unwrap();
                }
        */
    }

    for row in grid {
        for c in row {
            print!("{}", c);
        }
        print!("\n");
    }
}
