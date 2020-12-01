use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let input = BufReader::new(File::open("input")?);

    let mut xs: Vec<u16> = vec![];
    for res in input.lines() {
        xs.push(res?.parse().unwrap());
    }
    xs.sort_unstable();
    let mut lo = 0;
    let mut hi = xs.len() - 1;
    let it = loop {
        match (xs[lo] + xs[hi]).cmp(&2020) {
            Ordering::Less => {
                lo += 1;
            }
            Ordering::Greater => {
                hi -= 1;
            }
            Ordering::Equal => {
                break xs[lo] as u32 * xs[hi] as u32;
            }
        }
    };
    println!("{}", it);
    Ok(())
}
