use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Scope;

#[derive(Clone, Copy)]
pub struct Value<'t> {
    pub scope: &'t Scope,
    pub idx: usize,
    pub val: f64,
}

impl<'t> Add for Value<'t> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs_partial = 1.0;
        let rhs_partial = 1.0;
        let lhs_idx = self.idx;
        let rhs_idx = rhs.idx;
        let new_value = self.val + rhs.val;

        self.scope
            .op(lhs_partial, rhs_partial, lhs_idx, rhs_idx, new_value)
    }
}

impl<'t> Mul for Value<'t> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.scope
            .op(rhs.val, self.val, self.idx, rhs.idx, self.val * rhs.val)
    }
}

impl<'t> Sub for Value<'t> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.scope
            .op(1.0, -1.0, self.idx, rhs.idx, self.val - rhs.val)
    }
}

impl<'t> Neg for Value<'t> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.scope.op(-1.0, 0.0, self.idx, 0, -self.val)
    }
}

impl<'t> Div for Value<'t> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.scope.op(
            1.0 / rhs.val,
            -self.val / (rhs.val * rhs.val),
            self.idx,
            rhs.idx,
            self.val / rhs.val,
        )
    }
}

impl<'t> Mul<Value<'t>> for f64 {
    type Output = Value<'t>;

    fn mul(self, rhs: Value<'t>) -> Self::Output {
        rhs.scope.constant_op(self, rhs.idx, self * rhs.val)
    }
}
