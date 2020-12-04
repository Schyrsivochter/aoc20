use regex::Regex;
use std::ops::RangeInclusive;

fn name_to_idx(name: &str) -> usize {
    match name {
        "byr" => 0,
        "ecl" => 1,
        "eyr" => 2,
        "hcl" => 3,
        "hgt" => 4,
        "iyr" => 5,
        "pid" => 6,
        _ => usize::MAX,
    }
}

fn main() {
    let re_a = Regex::new(r"(\w{3}):(\S+)").unwrap();
    let res_b = [
        r"^[0-9]{4}$",
        r"^(amb|blu|brn|gry|grn|hzl|oth)$",
        r"^[0-9]{4}$",
        r"^#[0-9a-f]{6}$",
        r"^((?P<cm>[0-9]{3})cm|(?P<in>[0-9]{2})in)$",
        r"^[0-9]{4}$",
        r"^[0-9]{9}$",
    ].iter().copied()
        .map(Regex::new)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let input = std::fs::read_to_string("input").unwrap();

    let num_valid_a: u32 = input.split("\n\n")
        .map(|entry| {
            let mut presence = [false; 7];
            for caps in re_a.captures_iter(entry) {
                if let i @ 0..=7 = name_to_idx(&caps[1]) {
                    presence[i] = true;
                }
            }
            presence.iter().all(|&x| x) as u32
        })
        .sum();

    let num_valid_b: u32 = input.split("\n\n")
        .map(|entry| {
            let mut valid = [false; 7];
            for caps in re_a.captures_iter(entry) {
                if let i @ 0..=7 = name_to_idx(&caps[1]) {
                    valid[i] = validate(&res_b, i, &caps[2])
                }
            }
            valid.iter().all(|&x| x) as u32
        })
        .sum();

    println!("Answer for A: {}", num_valid_a);
    println!("Answer for B: {}", num_valid_b);
}

fn validate(res: &[Regex], field: usize, value: &str) -> bool {
    (field, value, &res[field]);
    let validate_range = |range: RangeInclusive<u16>, val| {
        res[field].is_match(val) && range.contains(&val.parse().unwrap())
    };

    match field {
        4 => { // HGT
            let caps = if let Some(caps) = res[field].captures(value) {
                caps
            } else {
                return false
            };
            if let Some(val) = caps.name("cm") {
                (150..=193).contains(&val.as_str().parse().unwrap())
            } else if let Some(val) = caps.name("in") {
                (59..=76).contains(&val.as_str().parse().unwrap())
            } else { unreachable!() }
        }
        0 => validate_range(1920..=2002, value), // BYR
        2 => validate_range(2020..=2030, value), // EYR
        5 => validate_range(2010..=2020, value), // IYR
        _ => res[field].is_match(value),
    }
}