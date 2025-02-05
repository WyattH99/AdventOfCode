use cgmath::Vector2;
use regex::Regex;
use std::fs;

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

    // Move each robot 100 moves in the future
    let robots: Vec<_> = robots
        .iter()
        .map(|robot| {
            // robot.0 is position
            // robot.1 is velocity
            let pos = robot.0 + robot.1 * 100;
            Vec2::new(pos.x.rem_euclid(WIDTH), pos.y.rem_euclid(HEIGHT))
        })
        .collect();

    // Filter the robots to the quadrants
    let robots: Vec<_> = robots
        .iter()
        .flat_map(|robot| {
            if robot.x == WIDTH / 2 || robot.y == HEIGHT / 2 {
                None
            } else {
                if robot.x < WIDTH / 2 {
                    if robot.y < HEIGHT / 2 {
                        Some(0)
                    } else {
                        Some(1)
                    }
                } else {
                    if robot.y < HEIGHT / 2 {
                        Some(2)
                    } else {
                        Some(3)
                    }
                }
            }
        })
        .collect();

    let sum = robots.iter().filter(|&&i| i == 0).count()
        * robots.iter().filter(|&&i| i == 1).count()
        * robots.iter().filter(|&&i| i == 2).count()
        * robots.iter().filter(|&&i| i == 3).count();

    println!("sum: {}", sum);
    //sum: 230230304 too low
    //sum: 232589280
}
