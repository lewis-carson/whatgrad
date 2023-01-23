use crate::{value::NumLike, Value};

impl<T: NumLike> Value<'_, T> {
    pub fn backprop(&self) -> Grad<T> {
        let tape_len = self.scope.nodes.borrow().len();
        let mut grad = vec![T::zero(); tape_len];
        grad[self.idx] = T::one();

        for i in (0..tape_len).rev() {
            let node = self.scope.nodes.borrow()[i];

            let lhs = node.parents_partials[0];
            let rhs = node.parents_partials[1];

            let g = grad[i];

            grad[lhs.0] += lhs.1 * g;
            grad[rhs.0] += rhs.1 * g;
        }

        Grad { grad }
    }
}

#[derive(Debug)]
pub struct Grad<T> {
    pub grad: Vec<T>,
}

impl<T: NumLike> Grad<T> {
    pub fn wrt(&self, value: Value<T>) -> T {
        self.grad[value.idx]
    }
}
