//use std::collections::VecDeque;
use std::fs;

fn main() {
    //let data = fs::read_to_string("example.txt").unwrap();
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    //println!("data: {}\n", data);

    // Tranform data string to a Vec of tuples with the
    // size and index of file and empty spaces
    let mut layout: Vec<_> = data
        .chars()
        .enumerate()
        .map(|(i, c)| {
            // parsing char to a digit base 10 as a usize
            let base = 10;
            let size = c.to_digit(base).unwrap() as usize;
            if i % 2 == 0 {
                // File
                (size, Some(i / 2), false)
            } else {
                // Free Space
                (size, None, false)
            }
        })
        .collect();
    //println!("layout: {:?}\n", layout);

    // Double pointer method to swap each
    for i in (0..layout.len()).rev() {
        if layout[i].1.is_none() || layout[i].2 {
            continue;
        }
        for j in 0..i {
            if layout[j].1.is_some() || layout[j].0 < layout[i].0 {
                continue;
            }

            let diff = layout[j].0 - layout[i].0;

            layout[j] = (layout[i].0, layout[i].1.take(), true);

            if diff > 0 {
                layout.insert(j + 1, (diff, None, true));
            }
            break;
        }
    }
    //println!("layout: {:?}\n", layout);

    let dense_format: Vec<_> = layout
        .iter()
        .flat_map(|(size, file_num, _)| std::iter::repeat(file_num).take(*size))
        .collect();
    //println!("dense: {:?}\n", dense_format);

    let checksum: usize = dense_format
        .iter()
        .enumerate()
        .map(|(i, f)| f.map(|f| i * f))
        .flatten()
        .sum();
    println!("checksum: {}", checksum);
    // checksum: 6291146824486
    // checksum: 6307279963620
}
