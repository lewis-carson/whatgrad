use crate::Value;

impl Value<'_> {
    pub fn backprop(&self) -> Grad {
        let tape_len = self.scope.nodes.borrow().len();
        let mut grad = vec![0.0; tape_len];
        grad[self.idx] = 1.0;

        for i in (0..tape_len).rev() {
            let node = self.scope.nodes.borrow()[i];

            let lhs = node.parents_partials[0];
            let rhs = node.parents_partials[1];

            grad[lhs.0] += lhs.1 * grad[i];
            grad[rhs.0] += rhs.1 * grad[i];
        }

        Grad { grad }
    }
}

#[derive(Debug)]
pub struct Grad {
    pub grad: Vec<f64>,
}

impl Grad {
    pub fn wrt(&self, value: Value) -> f64 {
        self.grad[value.idx]
    }
}
