#[rustfmt::skip]
pub mod tree4;

use crate::trees::Maybe32::{Decimal, Integer, Nothing};
use std::ops::{Add, Div, Mul, Sub};
pub use tree4::{V2O4, V3O4, V4O4};
use crate::AST;

pub trait Pow<Rhs = Self> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait Surd<Rhs = Self> {
    type Output;
    fn surd(self, rhs: Rhs) -> Self::Output;
}

#[derive(Copy, Clone, Debug)]
pub enum Maybe32 {
    Nothing,
    Integer(i32),
    Decimal(f32),
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> AST<T> {
    pub fn eval(self) -> T {
        match self {
            AST::Number(n) => { n }
            AST::Plus(a, b) => { a.eval() + b.eval() }
            AST::Minus(a, b) =>{ a.eval() - b.eval() }
            AST::Times(a, b) => { a.eval() * b.eval() }
            AST::Divide(a, b) => { a.eval() / b.eval() }
        }
    }
}

impl From<i32> for Maybe32 {
    fn from(i: i32) -> Self {
        Integer(i)
    }
}

impl Add<Maybe32> for Maybe32 {
    type Output = Maybe32;
    fn add(self, rhs: Maybe32) -> Self::Output {
        match self {
            Nothing => Nothing,
            Integer(a) => match rhs {
                Nothing => Nothing,
                Integer(b) => Integer(a + b),
                Decimal(b) => Decimal(a as f32 + b),
            },
            Decimal(a) => match rhs {
                Nothing => Nothing,
                Integer(b) => Decimal(a + b as f32),
                Decimal(b) => Decimal(a + b),
            },
        }
    }
}

impl Sub<Maybe32> for Maybe32 {
    type Output = Maybe32;
    fn sub(self, rhs: Maybe32) -> Self::Output {
        match self {
            Nothing => Nothing,
            Integer(a) => match rhs {
                Nothing => Nothing,
                Integer(b) => Integer(a - b),
                Decimal(b) => Decimal(a as f32 - b),
            },
            Decimal(a) => match rhs {
                Nothing => Nothing,
                Integer(b) => Decimal(a - b as f32),
                Decimal(b) => Decimal(a - b),
            },
        }
    }
}

impl Mul<Maybe32> for Maybe32 {
    type Output = Maybe32;
    fn mul(self, rhs: Maybe32) -> Self::Output {
        match self {
            Nothing => Nothing,
            Integer(a) => match rhs {
                Nothing => Nothing,
                Integer(b) => Integer(a * b),
                Decimal(b) => Decimal(a as f32 * b),
            },
            Decimal(a) => match rhs {
                Nothing => Nothing,
                Integer(b) => Decimal(a * b as f32),
                Decimal(b) => Decimal(a * b),
            },
        }
    }
}

impl Div<Maybe32> for Maybe32 {
    type Output = Maybe32;
    fn div(self, rhs: Maybe32) -> Self::Output {
        match rhs {
            Nothing => Nothing,
            Integer(b) => {
                if b == 0 {
                    Nothing
                } else {
                    match self {
                        Nothing => Nothing,
                        Integer(a) => Decimal(a as f32 / b as f32),
                        Decimal(a) => Decimal(a / b as f32),
                    }
                }
            }
            Decimal(b) => {
                if b == 0.0 {
                    Nothing
                } else {
                    match self {
                        Nothing => Nothing,
                        Integer(a) => Decimal(a as f32 / b),
                        Decimal(a) => Decimal(a / b),
                    }
                }
            }
        }
    }
}

impl PartialEq<i32> for Maybe32 {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Nothing => false,
            Integer(lhs) => lhs == other,
            Decimal(lhs) => (lhs * 10000.0) as i64 == (other * 10000) as i64,
        }
    }
}
