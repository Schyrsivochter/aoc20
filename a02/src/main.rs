use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() {
    let re = regex::Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let input = BufReader::new(File::open("input").unwrap());

    let num_valid_passwds: u16 = input.lines().map(Result::unwrap).map(|line| {
        let caps = re.captures(&line).unwrap();
        let lower: u16 = caps[1].parse().unwrap();
        let upper: u16 = caps[2].parse().unwrap();
        let c = caps[3].chars().next().unwrap();
        let passwd = &caps[4];

        let num = passwd.matches(c).count() as u16;
        (lower..=upper).contains(&num) as u16
    }).sum();

    println!("Answer: {}", num_valid_passwds);
}
