use std::cell::RefCell;

use crate::Value;

type ParentsPartials = (usize, f64);

#[derive(Clone, Copy)]
pub struct Node {
    pub parents_partials: [ParentsPartials; 2],
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
            parents_partials: [(len, 0.0), (len, 0.0)],
        });
        Value {
            scope: self,
            idx: len,
            val: value,
        }
    }

    pub fn op(
        &self,
        lhs_partial: f64,
        rhs_partial: f64,
        lhs_idx: usize,
        rhs_idx: usize,
        new_value: f64,
    ) -> Value {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            parents_partials: [(lhs_idx, lhs_partial), (rhs_idx, rhs_partial)],
        });
        Value {
            scope: self,
            idx: len,
            val: new_value,
        }
    }

    pub fn constant_op(&self, partial: f64, idx: usize, new_value: f64) -> Value {
        let len = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
            parents_partials: [(idx, partial), (len, 0.0)],
        });
        Value {
            scope: self,
            idx: len,
            val: new_value,
        }
    }
}
