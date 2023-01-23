use std::cell::RefCell;

use crate::Value;

#[derive(Clone, Copy)]
pub struct Node {
    pub partials: [f64; 2],
    pub parents: [usize; 2],
}

pub struct Scope {
    pub nodes: RefCell<Vec<Node>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            nodes: RefCell::new(Vec::<Node>::new()),
        }
    }

    pub fn value(&self, value: f64) -> Value {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            partials: [0.0, 0.0],
            // for a single (input) variable, we point the parents to itself
            parents: [len, len],
        });
        Value {
            tape: self,
            index: len,
            v: value,
        }
    }

    pub fn binary_op(
        &self,
        lhs_partial: f64,
        rhs_partial: f64,
        lhs_index: usize,
        rhs_index: usize,
        new_value: f64,
    ) -> Value {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            partials: [lhs_partial, rhs_partial],
            // for a single (input) variable, we point the parents to itself
            parents: [lhs_index, rhs_index],
        });
        Value {
            tape: self,
            index: len,
            v: new_value,
        }
    }

    /// Add a new node to the tape, where the node represents
    /// the result from a unary operation
    pub fn unary_op(&self, partial: f64, index: usize, new_value: f64) -> Value {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            partials: [partial, 0.0],
            // only the left index matters; the right index points to itself
            parents: [index, len],
        });
        Value {
            tape: self,
            index: len,
            v: new_value,
        }
    }
}
