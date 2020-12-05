use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut seat_list = input.lines()
        .map(|line| {
            line.chars()
                .fold(0u16, |hi, c| (hi << 1) + match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!("invalid input: {}", c),
                })
        })
        .collect_vec();

    seat_list.sort_unstable();

    let &highest = seat_list.iter().next_back().unwrap();

    let seat_id = seat_list.into_iter()
        .tuple_windows()
        .filter_map(|(lo, hi)| {
            if hi - lo == 2 {
                Some(lo + 1)
            } else {
                None
            }
        })
        .next().expect("Could not find seat");

    println!("Answer for A: {}", highest);
    println!("Answer for B: {}", seat_id);
}
