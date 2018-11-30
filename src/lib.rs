#![feature(box_syntax)]

pub(crate) mod trees;
pub(crate) mod utils;

pub use utils::solvable4;

pub enum AST<T> {
    Number(T),
    Plus(Box<AST<T>>, Box<AST<T>>),
    Minus(Box<AST<T>>, Box<AST<T>>),
    Times(Box<AST<T>>, Box<AST<T>>),
    Divide(Box<AST<T>>, Box<AST<T>>),
}

// Plus[Plus[Plus[Slot[1], Slot[2]], Slot[3]], Slot[4]]
// Plus(box Plus(box Plus(box Number(a1), box Number(a2)), box Number(a3)), box Number(a4))
