use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_traits::{NumAssignOps, NumOps, One, ToPrimitive, Zero};

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

// IMPLEMENTING AGAINST CONSTANT Ts
// WE HAVE TO IMPL EACH WAY ROUND

// ADDITION
impl<'t, T: NumLike> Add<Value<'t, T>> for (T,) {
    type Output = Value<'t, T>;

    fn add(self, rhs: Value<'t, T>) -> Self::Output {
        rhs.scope.constant_op(self.0, rhs.idx, rhs.val + self.0)
    }
}

impl<'t, T: NumLike> Add<T> for Value<'t, T> {
    type Output = Value<'t, T>;

    fn add(self, rhs: T) -> Self::Output {
        self.scope.constant_op(rhs, self.idx, rhs + self.val)
    }
}

// SUBTRACTION
impl<'t, T: NumLike> Sub<Value<'t, T>> for (T,) {
    type Output = Value<'t, T>;

    fn sub(self, rhs: Value<'t, T>) -> Self::Output {
        rhs.scope.constant_op(self.0, rhs.idx, self.0 - rhs.val)
    }
}

impl<'t, T: NumLike> Sub<T> for Value<'t, T> {
    type Output = Value<'t, T>;

    fn sub(self, rhs: T) -> Self::Output {
        self.scope.constant_op(rhs, self.idx, self.val - rhs)
    }
}

// MULTIPLICATION
impl<'t, T: NumLike> Mul<Value<'t, T>> for (T,) {
    type Output = Value<'t, T>;

    fn mul(self, rhs: Value<'t, T>) -> Self::Output {
        rhs.scope.constant_op(self.0, rhs.idx, rhs.val * self.0)
    }
}

impl<'t, T: NumLike> Mul<T> for Value<'t, T> {
    type Output = Value<'t, T>;

    fn mul(self, rhs: T) -> Self::Output {
        self.scope.constant_op(rhs, self.idx, rhs * self.val)
    }
}

// DIVISION
impl<'t, T: NumLike> Div<T> for Value<'t, T> {
    type Output = Value<'t, T>;

    fn div(self, rhs: T) -> Self::Output {
        self.scope
            .constant_op(T::one() / rhs, self.idx, self.val / rhs)
    }
}
