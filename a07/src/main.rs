use std::cell::{Cell, RefCell};
use regex::Regex;
use std::collections::{BTreeMap, VecDeque};

#[derive(Default, Debug)]
struct Bag<'c> {
    visited: Cell<bool>,
    contains: Vec<(u32, &'c str)>,
    contained_in: RefCell<Vec<&'c str>>,
}

impl<'c> Bag<'c> {
    fn new(contains: Vec<(u32, &'c str)>) -> Self {
        Bag {
            contains,
            ..Default::default()
        }
    }
}

fn main() {
    let line_re = Regex::new(r"(?P<col>\w+ \w+) bags contain ((?P<no>no other bags)|(?P<yes>.*))").unwrap();
    let yes_re = Regex::new(r"(?P<num>\d+) (?P<col>\w+ \w+) bags?[.,]").unwrap();

    let input = std::fs::read_to_string("input").unwrap();

    let bags = input.lines()
        .map(|line| {
            let caps = line_re.captures(line).expect("invalid input");
            let col = caps.name("col").unwrap().as_str();

            let bag = if let Some(_) = caps.name("no") {
                Default::default()
            } else if let Some(mat) = caps.name("yes") {
                let contains = yes_re.captures_iter(mat.as_str())
                    .map(|caps| (
                        caps["num"].parse().unwrap(),
                        caps.name("col").unwrap().as_str()
                    ))
                    .collect();
                Bag::new(contains)
            } else {
                panic!("invalid input")
            };
            (col, bag)
        })
        .collect::<BTreeMap<&str, Bag>>();

    for (&col, bag) in &bags {
        for (_, contained) in &bag.contains {
            bags.get(contained).expect("invalid bag reference")
                .contained_in.borrow_mut()
                .push(col);
        }
    }

    // Part A
    let mine = "shiny gold";
    let mut containers_a = 0u32;
    let mut q = VecDeque::new();
    q.push_back(mine);

    while let Some(col) = q.pop_front() {
        let bag = &bags[col];
        if bag.visited.get() { continue }
        containers_a += 1;
        for &container in &*bag.contained_in.borrow_mut() {
            q.push_back(container)
        }
        bag.visited.set(true);
    }
    containers_a -= 1; // shiny gold itself does not count

    println!("Answer for A: {}", containers_a);

    // Part B

    let b = contained(&bags, mine);
    println!("Answer for B: {}", b);

}

fn contained<'c>(bags: &BTreeMap<&'c str, Bag>, col: &'c str) -> u32 {
    bags[col].contains.iter()
        .map(|&(n, col_inner)| {
            n * (1 + contained(bags, col_inner))
        })
        .sum()
}

