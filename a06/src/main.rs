fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let num_yes_a: usize = input.split("\n\n")
        .map(|entry| {
            let mut qs = [false; 26];
            entry.chars()
                .for_each(|c| match c {
                    'a'..='z' => qs[c as usize - 'a' as usize] = true,
                    _ => (),
                });
            qs.iter().filter(|x| **x).count()
        })
        .sum();

    let num_yes_b: usize = input.split("\n\n")
        .map(|entry| {
            let mut qs = [true; 26];
            for line in entry.lines() {
                let mut these_qs = [false; 26];
                for c in line.chars() {
                    match c {
                        'a'..='z' => these_qs[c as usize - 'a' as usize] = true,
                        _ => (),
                    }
                }
                for (q, this_q) in qs.iter_mut().zip(&these_qs) {
                    *q &= this_q;
                }
            }
            qs.iter().filter(|x| **x).count()
        })
        .sum();

    println!("Answer for A: {}", num_yes_a);
    println!("Answer for B: {}", num_yes_b);
}
