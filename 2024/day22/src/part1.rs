use std::fs;

/*
* Secret number sequences:
* - Each evolves to the next
* - Calculate by:
*   - secret * 64 => mix => prune
*   - secret / 32 => floor => mix => prune
*   - secret * 2048 => mix => prune
*
* - mix: is bitwise xor
*  - 42 xor 15 => 37
* - prune: is modulo
*  - secret mod 16777216
*
*  The whole Calc:
*  sec = ((sec * 64) ^ sec) mod 16777216
*  sec = (((sec / 32) floor) ^ sec) mod 16777216
*  sec = ((sec * 2048) ^ sec) mod 16777216
*/

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}", data);

    /*
     *  The whole Calc:
     *  sec = ((sec * 64) ^ sec) mod 16777216
     *  sec = (((sec / 32) floor) ^ sec) mod 16777216
     *  sec = ((sec * 2048) ^ sec) mod 16777216

    let mut sec: isize = 123;
    for i in 0..10 {
        sec = ((sec * 64) ^ sec) % 16777216;
        sec = ((sec / 32) ^ sec) % 16777216;
        sec = ((sec * 2048) ^ sec) % 16777216;
        println!("{} : {}", i, sec);
    }
    */

    let buyers: Vec<isize> = data.lines().map(|x| x.parse().unwrap()).collect();
    //println!("buyers: {:?}", buyers);
    let sec_nums: Vec<isize> = buyers
        .iter()
        .map(|x| {
            let mut sec: isize = *x;
            for _ in 0..2000 {
                sec = ((sec * 64) ^ sec) % 16777216;
                sec = ((sec / 32) ^ sec) % 16777216;
                sec = ((sec * 2048) ^ sec) % 16777216;
            }
            sec
        })
        .collect();
    //println!("sec_nums: {:?}", sec_nums);
    println!("sum: {}", sec_nums.iter().sum::<isize>());
    // sum: 14869099597
}
