use game24::{Maybe32, AST};

#[derive(Clone, Copy, Debug)]
enum Operator {
    Minus,
    Plus,
    Times,
    Divide,
}

#[derive(Clone, Debug)]
struct Factor {
    pub ast: AST<Maybe32>,
    pub value: Maybe32,
}

fn apply(op: Operator, left: &[Factor], right: &[Factor]) -> Vec<Factor> {
    let mut ret = Vec::new();
    for l in left.iter() {
        for r in right.iter() {
            use Operator::*;
            ret.push(match op {
                Minus => Factor { ast: AST::new("-", &l.ast, &r.ast), value: l.value - r.value },
                Plus => Factor { ast: AST::new("+", &l.ast, &r.ast), value: l.value + r.value },
                Times => Factor { ast: AST::new("*", &l.ast, &r.ast), value: l.value * r.value },
                Divide => Factor { ast: AST::new("/", &l.ast, &r.ast), value: l.value / r.value },
            })
        }
    }
    ret
}

fn calc(op: [Operator; 3], numbers: [Maybe32; 4]) -> Vec<Factor> {
    fn calc(op: &[Operator], numbers: &[Maybe32], acc: &[Factor]) -> Vec<Factor> {
        use Operator::*;
        if op.is_empty() {
            return Vec::from(acc);
        }
        let mut ret = Vec::new();
        let mono_factor = [Factor { ast: AST::Number(numbers[0]), value: numbers[0] }];
        match op[0] {
            Times => ret.extend_from_slice(&apply(op[0], acc, &mono_factor)),
            Divide => {
                ret.extend_from_slice(&apply(op[0], acc, &mono_factor));
                ret.extend_from_slice(&apply(op[0], &mono_factor, acc));
            }
            Minus => {
                ret.extend_from_slice(&apply(op[0], acc, &mono_factor));
                ret.extend_from_slice(&apply(op[0], &mono_factor, acc));
            }
            Plus => ret.extend_from_slice(&apply(op[0], acc, &mono_factor)),
        }
        calc(&op[1..], &numbers[1..], &ret)
    }
    calc(&op, &numbers[1..], &[Factor { ast: AST::Number(numbers[0]), value: numbers[0] }])
}

fn solutions(numbers: [Maybe32; 4], target: i32) -> Vec<Factor> {
    use std::collections::hash_set::HashSet;
    let mut ret = Vec::new();
    for ops in OpIter(0) {
        for o in orders().iter() {
            let numbers = apply_order(numbers, o);
            let r = calc(ops, numbers);
            ret.extend(r.into_iter().filter(|&Factor { value, ast: ref content }| value == 24))
        }
    }
    ret
}

fn main() {
    let numbers = [Maybe32::from(1), Maybe32::from(2), Maybe32::from(3), Maybe32::from(4)];
    let solutions = solutions(numbers, 24);
    let len = solutions.len();
    if len == 0 {
        println!("no solution for {:?}, {:?}, {:?}, {:?}", numbers[0], numbers[1], numbers[2], numbers[3]);
        return;
    }
    println!("solutions for {:?}, {:?}, {:?}, {:?}", numbers[0], numbers[1], numbers[2], numbers[3]);
    for s in solutions {
        println!("{:?}", s.ast)
    }
    println!("{} solutions found", len);
    return;
}

struct OpIter(usize);

impl Iterator for OpIter {
    type Item = [Operator; 3];
    fn next(&mut self) -> Option<[Operator; 3]> {
        use Operator::*;
        const OPTIONS: [Operator; 4] = [Times, Minus, Plus, Divide];
        if self.0 >= 1 << 6 {
            return None;
        }
        let f1 = OPTIONS[(self.0 & (3 << 4)) >> 4];
        let f2 = OPTIONS[(self.0 & (3 << 2)) >> 2];
        let f3 = OPTIONS[(self.0 & (3 << 0)) >> 0];
        self.0 += 1;
        Some([f1, f2, f3])
    }
}

fn orders() -> [[usize; 4]; 24] {
    [
        [0, 1, 2, 3],
        [0, 1, 3, 2],
        [0, 2, 1, 3],
        [0, 2, 3, 1],
        [0, 3, 1, 2],
        [0, 3, 2, 1],
        [1, 0, 2, 3],
        [1, 0, 3, 2],
        [1, 2, 0, 3],
        [1, 2, 3, 0],
        [1, 3, 0, 2],
        [1, 3, 2, 0],
        [2, 0, 1, 3],
        [2, 0, 3, 1],
        [2, 1, 0, 3],
        [2, 1, 3, 0],
        [2, 3, 0, 1],
        [2, 3, 1, 0],
        [3, 0, 1, 2],
        [3, 0, 2, 1],
        [3, 1, 0, 2],
        [3, 1, 2, 0],
        [3, 2, 0, 1],
        [3, 2, 1, 0],
    ]
}

fn apply_order(numbers: [Maybe32; 4], order: &[usize; 4]) -> [Maybe32; 4] {
    [numbers[order[0]], numbers[order[1]], numbers[order[2]], numbers[order[3]]]
}
