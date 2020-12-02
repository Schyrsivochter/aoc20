use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() {
    let re = regex::Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let input = BufReader::new(File::open("input").unwrap());

    let num_valid_passwds: usize = input.lines().map(Result::unwrap).filter_map(|line| {
        let caps = re.captures(&line).unwrap();
        let first = caps[1].parse::<usize>().unwrap() - 1;
        let second = caps[2].parse::<usize>().unwrap() - 1;
        let c = caps[3].chars().next().unwrap();

        let mut passwd_chars = caps[4].chars().peekable();

        if first != 0 { passwd_chars.nth(first - 1); } // passwd_chars.advance_by(first)?;

        let &first_c = passwd_chars.peek()?;
        let second_c = passwd_chars.nth(second - first).unwrap_or(' ');
        if (first_c == c) ^ (second_c == c) {
            Some(())
        } else {
            None
        }
    }).count();

    println!("Answer: {}", num_valid_passwds);
}
