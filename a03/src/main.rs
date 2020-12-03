use std::io::{prelude::*, BufReader};
use std::fs::File;

fn main() {
    let input = BufReader::new(File::open("input").unwrap());

    let map = input.lines().map(Result::unwrap).map(|line: String| {
        line.chars().map(|c| c == '#').collect::<Vec<_>>().into_boxed_slice()
    }).collect::<Vec<_>>();
    let num_trees_a = count_trees(&map, 3, 1);

    let answer_b = num_trees_a
        * count_trees(&map, 1, 1)
        * count_trees(&map, 5, 1)
        * count_trees(&map, 7, 1)
        * count_trees(&map, 1, 2);

    println!("Answer for A: {}", num_trees_a);
    println!("Answer for B: {}", answer_b)
}

fn count_trees(map: &[Box<[bool]>], x_step: usize, y_step: usize) -> u32 {
    let width = map[0].len();

    map.iter()
        .step_by(y_step)
        .zip(
            (0..).step_by(x_step).map(|x| x % width)
        )
        .map(|(row, x)| row[x] as u32)
        .sum()
}
