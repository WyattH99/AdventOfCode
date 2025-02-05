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

    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'A' {
                let (i, j) = (i as isize, j as isize);

                if let (Some(&tl), Some(&tr), Some(&bl), Some(&br)) = (
                    grid.iget(i-1).and_then(|e|e.iget(j-1)),
                    grid.iget(i+1).and_then(|e|e.iget(j-1)),
                    grid.iget(i-1).and_then(|e|e.iget(j+1)),
                    grid.iget(i+1).and_then(|e|e.iget(j+1))
                ) {
                    let d0 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
                    let d1 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');
                    if d0 && d1 {
                        result += 1;
                    }
                }
            }
        }
    }


    println!("result: {}", result);
    // result: 1992

}
