use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_traits::{NumAssignOps, NumOps, One, Zero};

use crate::Scope;

pub trait NumLike: Clone + Copy + NumOps + NumAssignOps + Zero + One + Display {}
impl<T: Clone + Copy + NumOps + NumAssignOps + Zero + One + Display> NumLike for T {}

#[derive(Clone, Copy)]
pub struct Value<'t, T> {
    pub scope: &'t Scope<T>,
    pub idx: usize,
    pub val: T,
}

impl<'t, T: NumLike> Add for Value<'t, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs_partial = T::one();
        let rhs_partial = T::one();
        let lhs_idx = self.idx;
        let rhs_idx = rhs.idx;
        let new_value = self.val + rhs.val;

        self.scope
            .op(lhs_partial, rhs_partial, lhs_idx, rhs_idx, new_value)
    }
}

impl<'t, T: NumLike> Mul for Value<'t, T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.scope
            .op(rhs.val, self.val, self.idx, rhs.idx, self.val * rhs.val)
    }
}

impl<'t, T: NumLike + Neg<Output = T>> Sub for Value<'t, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.scope
            .op(T::one(), -T::one(), self.idx, rhs.idx, self.val - rhs.val)
    }
}

impl<'t, T: NumLike + Neg<Output = T>> Neg for Value<'t, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.scope.op(-T::one(), T::zero(), self.idx, 0, -self.val)
    }
}

impl<'t, T: NumLike + Neg<Output = T>> Div for Value<'t, T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.scope.op(
            T::one() / rhs.val,
            -self.val / (rhs.val * rhs.val),
            self.idx,
            rhs.idx,
            self.val / rhs.val,
        )
    }
}
