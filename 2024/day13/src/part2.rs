use std::fs;

use cgmath::Vector2;
use regex::Regex;

type Vec2 = Vector2<isize>;

struct Equation {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

impl Equation {
    fn count_tokens(&self) -> Option<usize> {
        /*
        *   Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        *
        * System of Equations
        * Only one Solution
        * 8400 = A*94 + B*22
        * 5400 = A*34 + B*67
        */
        let p0 = self.prize.x;
        let a0 = self.a.x;
        let b0 = self.b.x;
        let p1 = self.prize.y;
        let a1 = self.a.y;
        let b1 = self.b.y;

        // p0 = A * a0 + B * b0
        // p1 = A * a1 + B * b1
        // Using Wolfram Alpha to solve the SoE
        // solve p1 = A * a1 + B * b1,p0 = A * a0 + B * b0 for A,B
        // A = (b1*p0 - b0*p1)/(a0*b1 - a1*b0) and B = (a1*p0 - a0*p1)/(a1*b0 - a0*b1) and a1*b0!=a0*b1 and b0!=0
        //

        let a = (b1 * p0 - b0 * p1) / (a0 * b1 - a1 * b0);
        let b = (a1 * p0 - a0 * p1) / (a1 * b0 - a0 * b1);

        if p1 == a * a1 + b * b1 && p0 == a * a0 + b * b0 {
            Some((3 * a + b) as usize)
        } else {
            None
        }
    }
}

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("data: {}", data);

    let data: Vec<_> = data.lines().collect();

    let data: Vec<_> = data.split(|line| line.is_empty()).collect();

    let equations: Vec<_> = data
        .iter()
        .map(|lines| {
            let mut iter = lines.iter();
            let a = iter.next().map(|s| str_to_vec2(s)).unwrap();
            let b = iter.next().map(|s| str_to_vec2(s)).unwrap();
            let prize = iter.next().map(|s| str_to_vec2(s)).unwrap();
            let prize = Vec2::new(prize.x + 10000000000000, prize.y + 10000000000000);
            Equation { a, b, prize }
        })
        .collect();

    let sum: usize = equations.iter().filter_map(|e| e.count_tokens()).sum();

    println!("sum: {}", sum);
    // sum: 37901
    // sum: 77407675412647
}

fn str_to_vec2(s: &str) -> Vec2 {
    let r = Regex::new(r"(\d+).*?(\d+)").unwrap();
    let capture = r.captures_iter(s).next().unwrap();
    Vec2::new(capture[1].parse().unwrap(), capture[2].parse().unwrap())
}
