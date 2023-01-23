use crate::Value;

impl Value<'_> {
    pub fn backprop(&self) -> Grad {
        let tape_len = self.tape.nodes.borrow().len();
        let mut grad = vec![0.0; tape_len];
        grad[self.index] = 1.0;

        for i in (0..tape_len).rev() {
            let node = self.tape.nodes.borrow()[i];

            grad[node.parents[0]] += node.partials[0] * grad[i];
            grad[node.parents[1]] += node.partials[1] * grad[i];
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
        self.grad[value.index]
    }
}
