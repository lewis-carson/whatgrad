use std::ops::{Add, Div, Mul, Sub};

use crate::{value::NumLike, Value};

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
