use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Scope;

#[derive(Clone, Copy)]
pub struct Value<'t> {
    pub tape: &'t Scope,
    pub index: usize,
    pub v: f64,
}

impl<'t> Add for Value<'t> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.tape
            .binary_op(1.0, 1.0, self.index, rhs.index, self.v + rhs.v)
    }
}

impl<'t> Mul for Value<'t> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.tape
            .binary_op(rhs.v, self.v, self.index, rhs.index, self.v * rhs.v)
    }
}

impl<'t> Sub for Value<'t> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.tape
            .binary_op(1.0, -1.0, self.index, rhs.index, self.v - rhs.v)
    }
}

impl<'t> Neg for Value<'t> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.tape.binary_op(-1.0, 0.0, self.index, 0, -self.v)
    }
}

impl<'t> Div for Value<'t> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.tape.binary_op(
            1.0 / rhs.v,
            -self.v / (rhs.v * rhs.v),
            self.index,
            rhs.index,
            self.v / rhs.v,
        )
    }
}

impl<'t> Mul<Value<'t>> for f64 {
    type Output = Value<'t>;

    fn mul(self, rhs: Value<'t>) -> Self::Output {
        rhs.tape.unary_op(self, rhs.index, self * rhs.v)
    }
}
