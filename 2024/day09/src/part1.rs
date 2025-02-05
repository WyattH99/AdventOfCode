use std::collections::VecDeque;
use std::fs;

fn main() {
    let data = fs::read_to_string("example.txt").unwrap();
    //let data = fs::read_to_string("input.txt").unwrap();
    //println!("data: {}\n", data);
    let data = data.trim();

    // Tranform data string to a Vec of tuples with the
    // size and index of file and empty spaces
    let layout: Vec<_> = data
        .chars()
        .enumerate()
        .map(|(i, c)| {
            // parsing char to a digit base 10 as a usize
            let base = 10;
            let size = c.to_digit(base).unwrap() as usize;
            if i % 2 == 0 {
                // File
                (size, Some(i / 2))
            } else {
                // Free Space
                (size, None)
            }
        })
        .collect();
    println!("layout: {:?}\n", layout);

    // Take the layout and make a file queue
    let mut files_queue: VecDeque<_> = layout
        .iter()
        .flat_map(|(size, index)| index.map(|i| std::iter::repeat(i).take(*size)))
        .flatten()
        .collect();
    println!("files_queue: {:?}\n", files_queue);

    let mut dense_format = vec![];

    for block in layout.iter() {
        for _ in 0..block.0 {
            let d = if block.1.is_some() {
                files_queue.pop_front()
            } else {
                files_queue.pop_back()
            };
            if let Some(d) = d {
                dense_format.push(d);
            } else {
                break;
            }
        }
    }
    println!("dense_format: {:?}\n", dense_format);

    let checksum: usize = dense_format.iter().enumerate().map(|(i, f)| i * f).sum();
    println!("checksum: {}", checksum);
    // checksum: 6291146824486
}
