use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::cmp::Ordering;

fn main() -> Result<(), ()> {
    let input = BufReader::new(File::open("input").unwrap());

    let mut xs: Vec<u16> = vec![];
    for res in input.lines() {
        xs.push(res.unwrap().parse().unwrap());
    }
    xs.sort_unstable();

    for (third_i, &third) in xs.iter().enumerate() {
        let mut lo = 0;
        let mut hi = xs.len() - 1;
        while lo != hi {
            match (xs[lo] + xs[hi] + third).cmp(&2020) {
                Ordering::Less => {
                    lo += 1;
                }
                Ordering::Greater => {
                    hi -= 1;
                }
                Ordering::Equal => {
                    if lo == third_i { lo += 1; continue; }
                    if hi == third_i { hi -= 1; continue; }
                    let it = xs[lo] as u32 * xs[hi] as u32 * third as u32;
                    println!("Answer: {}", it);
                    return Ok(());
                }
            }
        };
    }
    eprintln!("Did not find an answer.");
    Err(())
}
