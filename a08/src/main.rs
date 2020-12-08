use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
enum Op {
    Jmp(i32),
    Acc(i32),
    Nop,
}

impl Op {
    fn from_line(line: &str) -> Self {
        match &line[..3] {
            "nop" => Op::Nop,
            "jmp" => Op::Jmp(line[4..].parse().unwrap()),
            "acc" => Op::Acc(line[4..].parse().unwrap()),
            x => panic!("Invalid op: {}", x),
        }
    }
}

#[derive(Debug, Clone)]
struct Instr {
    visited: Cell<bool>,
    op: Op,
}

impl From<Op> for Instr {
    fn from(op: Op) -> Self {
        Instr { op, visited: Cell::new(false) }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let code = input.lines()
        .map(|line| Op::from_line(line).into())
        .collect::<Vec<_>>().into_boxed_slice();

    let mut ip = 0isize;
    let mut acc = 0;
    let acc_before_loop = loop {
        let Instr { op, visited } = &code[ip as usize];

        if visited.get() {
            break acc;
        }

        match op {
            Op::Jmp(amt) => ip += *amt as isize,
            Op::Acc(amt) => {
                acc += amt;
                ip += 1;
            }
            Op::Nop => ip += 1,
        }
        visited.set(true);
    };

    println!("Answer for A: {}", acc_before_loop);
}
