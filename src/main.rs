use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f).lines();

    let mut i = 0;
    let mut s = 0;
    let mut v = Vec::new();

    for lines in &reader.chunks(3) {
        for line in lines.enumerate() {
            v.push(line.1?);
        }
        'outer: for c1 in v[i].chars() {
            for c2 in v[i + 1].chars() {
                if c1 == c2 {
                    for c3 in v[i + 2].chars() {
                        if c2 == c3 {
                            if (c1 as u32) < 96 {
                                s += c1 as u32 - 38;
                                // println!("{} {} {}", v[i], v[i + 1], v[i + 2]);
                                // println!("{} {} {}", c1, c1 as u32 - 38, s);
                            } else {
                                s += c1 as u32 - 96;
                                // println!("{} {} {}", v[i], v[i + 1], v[i + 2]);
                                // println!("{} {} {}", c1, c1 as u32 - 96, s);
                            }
                            i += 3;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    println!("Score: {}", s);

    Ok(())
}
