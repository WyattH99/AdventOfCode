use std::fs;
use extend::ext;

#[ext]
pub impl<T> Vec<T> {
    fn iget(&self, index: isize) -> Option<&T> {
        if index < 0 {
            None
        } else {
            self.get(index as usize)
        }
    }
}

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    //println!("{}", data);

    let grid: Vec<Vec<_>> = data.lines().map(|line| line.chars().collect()).collect();
    //println!("{:?}", grid);

    // Vector that holds the offset and the character for the rest of XMAS
    let mas: Vec<_> = "MAS"
        .chars()
        .enumerate()
        .map(|(i,c)| ((i+1) as isize, c))
        .collect();
    //println!("{:?}", mas);

    // Directions to scan that XMAS can be in
    let dirs = vec![
        (1, 1), 
        (1, 0), 
        (1, -1),
        (0, 1), 
        //(0, 0), Not moving and we already found the X 
        (0, -1),
        (-1, 1), 
        (-1, 0), 
        (-1, -1),
    ];

    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                let (i, j) = (i as isize, j as isize);

                for (dx, dy) in dirs.iter() {
                    if mas.iter()
                        .all(|(step, c)| {
                            let x = i + dx * step;
                            let y = j + dy * step;

                            grid.iget(x).and_then(|e| e.iget(y)) == Some(c)
                        }) {
                        result += 1;
                    }
                }
            }
        }
    }


    println!("result: {}", result);
    // result: 2571

}
