#![feature(box_syntax)]

pub(crate) mod trees;
pub(crate) mod utils;

pub use utils::solvable4;

#[derive(Copy, Clone, Debug)]
pub enum Maybe32 {
    Nothing,
    Integer(i32),
    Decimal(f32),
}

#[derive(Clone, Debug)]
pub enum AST<T: Clone> {
    Number(T),
    Plus(Box<AST<T>>, Box<AST<T>>),
    Minus(Box<AST<T>>, Box<AST<T>>),
    Times(Box<AST<T>>, Box<AST<T>>),
    Divide(Box<AST<T>>, Box<AST<T>>),
    Power(Box<AST<T>>, Box<AST<T>>),
    Surd(Box<AST<T>>, Box<AST<T>>),
}

// Plus[Plus[Plus[Slot[1], Slot[2]], Slot[3]], Slot[4]]
// Plus(box Plus(box Plus(box Number(a1), box Number(a2)), box Number(a3)), box Number(a4))
