use crate::{
    trees::{V2O4, V3O4, V4O4},
    Maybe32, AST,
};

pub fn solve(input: &[i32], target: i32) -> Option<AST<Maybe32>> {
    match input {
        [] => None,
        [a] => {
            if *a == target {
                Some(AST::Number(Maybe32::from(*a)))
            }
            else {
                None
            }
        }
        [a, b] => {
            for f in V2O4.iter() {
                let e = f(Maybe32::from(*a), Maybe32::from(*b));
                if e.clone().eval() == target {
                    return Some(e);
                }
            }
            return None;
        }
        [a, b, c] => {
            for f in V3O4.iter() {
                let e = f(Maybe32::from(*a), Maybe32::from(*b), Maybe32::from(*c));
                if e.clone().eval() == target {
                    return Some(e);
                }
            }
            return None;
        }
        [a, b, c, d] => {
            for f in V4O4.iter() {
                let e = f(Maybe32::from(*a), Maybe32::from(*b), Maybe32::from(*c), Maybe32::from(*d));
                if e.clone().eval() == target {
                    return Some(e);
                }
            }
            return None;
        }
        _ => unimplemented!(),
    }
}

pub fn solve_all(input: &[i32], target: i32) -> Vec<AST<Maybe32>> {
    let mut out = vec![];
    match input {
        [] => (),
        [a] => {
            if *a == target {
                out.push(AST::Number(Maybe32::from(*a)))
            }
        }
        [a, b] => {
            for f in V2O4.iter() {
                let e = f(Maybe32::from(*a), Maybe32::from(*b));
                if e.clone().eval() == target {
                    out.push(e)
                }
            }
        }
        [a, b, c] => {
            for f in V3O4.iter() {
                let e = f(Maybe32::from(*a), Maybe32::from(*b), Maybe32::from(*c));
                if e.clone().eval() == target {
                    out.push(e)
                }
            }
        }
        [a, b, c, d] => {
            for f in V4O4.iter() {
                let e = f(Maybe32::from(*a), Maybe32::from(*b), Maybe32::from(*c), Maybe32::from(*d));
                if e.clone().eval() == target {
                    out.push(e)
                }
            }
        }
        _ => unimplemented!(),
    }
    return out;
}

pub fn solvable(input: &[i32], target: i32) -> bool {
    match solve(input, target) {
        None => false,
        Some(_) => true,
    }
}
