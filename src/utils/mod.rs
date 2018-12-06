pub mod eval;

pub use eval::solvable4;

pub mod combinatorics;

use crate::AST;
pub use combinatorics::{cartesian_product, permutations, CartesianProduct, Permutations};
use std::ops::{Add, Div, Mul, Sub};

// o[n, o[n, o[n, n]]]

impl<T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> AST<T> {
    pub fn eval(self) -> T {
        match self {
            AST::Number(n) => n,
            AST::Plus(a, b) => a.eval() + b.eval(),
            AST::Minus(a, b) => a.eval() - b.eval(),
            AST::Times(a, b) => a.eval() * b.eval(),
            AST::Divide(a, b) => a.eval() / b.eval(),
            AST::Power(_, _) => unimplemented!(),
            AST::Surd(_, _) => unimplemented!(),
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            AST::Number(n) => n,
            _ => unreachable!(),
        }
    }
    pub fn new(o: &str, lhs: &AST<T>, rhs: &AST<T>) -> AST<T> {
        match o {
            "+" => AST::Plus(box (*lhs).clone(), box (*rhs).clone()),
            "-" => AST::Minus(box (*lhs).clone(), box (*rhs).clone()),
            "*" => AST::Times(box (*lhs).clone(), box (*rhs).clone()),
            "/" => AST::Divide(box (*lhs).clone(), box (*rhs).clone()),
            _ => unreachable!(),
        }
    }
}
