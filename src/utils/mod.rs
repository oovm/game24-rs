pub mod basic;

pub use basic::solvable;

#[allow(dead_code)]
pub mod combinatorics;

use crate::{Maybe32, AST};
pub use combinatorics::{cartesian_product, permutations, CartesianProduct, Permutations};
use std::{
    fmt,
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

// o[n, o[n, o[n, n]]]

impl Display for Maybe32 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Maybe32::Nothing => write!(f, "Nothing"),
            Maybe32::Integer(n) => write!(f, "{}", n),
            Maybe32::Decimal(n) => write!(f, "{}", n),
        }
    }
}

fn o2<T:Clone + Display>(ast: &Box<AST<T>>) -> String {
    match **ast {
        AST::Plus(..) | AST::Minus(..) => format!("({})", ast),
        _ => format!("{}", ast),
    }
}
fn o3<T:Clone + Display>(ast: &Box<AST<T>>) -> String {
    match **ast {
        AST::Plus(..) | AST::Minus(..) | AST::Times(..) | AST::Divide(..) => format!("({})", ast),
        _ => format!("{}", ast),
    }
}

impl<T: Clone + Display> Display for AST<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::Number(n) => write!(f, "{}", n),
            AST::Plus(l, r) => write!(f, "{} + {}", l, r),
            AST::Minus(l, r) => write!(f, "{} - {}", l, r),
            AST::Times(l, r) => write!(f, "{} × {}", o2(l), o2(r)),
            AST::Divide(l, r) => write!(f, "{} ÷ {}", o2(l), o2(r)),
            AST::Power(l, r) => write!(f, "{}^{}", o3(l), o3(r)),
            AST::Surd(l, r) => write!(f, "{}√{}", o3(l), o3(r)),
        }
    }
}

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
